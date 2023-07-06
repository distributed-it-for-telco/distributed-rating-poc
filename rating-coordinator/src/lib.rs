use rating_interface::{
    RatingAgent, RatingAgentSender, RatingCoordinator,
    RatingCoordinatorReceiver, RatingProcessRequest, RatingRequest, RatingResponse,
    RatingResponseBuilder, ValidationRequest, ValidationResponse,
};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::info;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, RatingCoordinator)]
struct RatingAgentCoordinatorActor {}

#[async_trait]
/// Implementation of Rating Coodinator
impl RatingCoordinator for RatingAgentCoordinatorActor {
    async fn handle_rating_process(
        &self,
        _ctx: &Context,
        _arg: &RatingProcessRequest,
    ) -> RpcResult<RatingResponse> {
        info!("Hello I'm your rating coordinator");
        info!("Current used agent is: {}", _arg.rating_request.agent_id);

        let mut rating_agents_stack: Vec<(String, String)> = Vec::new();

        let validation_response_as_rating = match handle_validation_cycle(_ctx, _arg, &mut rating_agents_stack).await {
            Ok(validation) => validation,
            Err(e) => return RpcResult::from(Err(e)),
        };

        if validation_response_as_rating.authorization_status.code == 401 {
            return RpcResult::Ok(validation_response_as_rating);
        }

        RpcResult::from(match handle_rating_cycle(_ctx, _arg, &mut rating_agents_stack).await {
            Ok(rating) => Ok(rating),
            Err(e) => Err(e),
        })
    }
}

async fn handle_validation_cycle(
    _ctx: &Context,
    rating_process_request: &RatingProcessRequest,
    rating_agents_stack: &mut Vec<(String, String)>
) -> RpcResult<RatingResponse> {
    if rating_process_request.headers.is_none() {
        return RpcResult::from(Err(RpcError::Other(
            "Can't validate client usage, client ip not found!".to_owned(),
        )));
    }

    let client_headers = rating_process_request.headers.to_owned().unwrap();
    let mut client_ip = "";
    let mut client_country = None;
    if client_headers.get("client_ip").is_some() {
        client_ip = client_headers.get("client_ip").unwrap();
    }

    if client_headers.get("client_country").is_some() {
        client_country = Some(client_headers.get("client_country").unwrap().to_string());
    }

    info!("Validating against agent: {}", rating_process_request.rating_request.agent_id);

    let mut validation_response = validate_through_agent(
        _ctx,
        &rating_process_request.rating_request,
        client_ip.to_string(),
        client_country.to_owned(),
    )
    .await?;

    rating_agents_stack.push((rating_process_request.rating_request.agent_id.to_owned(), rating_process_request.rating_request.customer_id.to_owned()));

    info!("Vendor validation status: {}", validation_response.valid);
    info!("Vendor validation have next agent and valid: {}",
        validation_response.valid && validation_response.next_agent != None);

    while validation_response.valid && validation_response.next_agent != None {
        let next_agent_name = validation_response.next_agent.to_owned().unwrap().name;
        let next_partner_id = validation_response
            .next_agent
            .to_owned()
            .unwrap()
            .partner_id;

        let mut updated_rating_request = rating_process_request.rating_request.clone();
        updated_rating_request.customer_id = next_partner_id;
        updated_rating_request.agent_id = next_agent_name;

        info!("Validating against agent: {}", validation_response.next_agent.to_owned().unwrap().name);

        rating_agents_stack.push((updated_rating_request.agent_id.to_owned(), updated_rating_request.customer_id.to_owned()));
        
        validation_response = validate_through_agent(
            _ctx,
            &updated_rating_request,
            client_ip.to_string(),
            client_country.to_owned(),
        )
        .await?;
    }

    let mut rating_response_builder = RatingResponseBuilder::new();

    if !validation_response.valid {
        rating_response_builder
            .message(&"Validation failed")
            .not_authorized();
    } else {
        rating_response_builder.message(&"Valid usage").authorized();
    }

    Ok(rating_response_builder.build())
}

async fn validate_through_agent(
    ctx: &Context,
    rating_request: &RatingRequest,
    client_ip: String,
    client_country: Option<String>,
) -> RpcResult<ValidationResponse> {
    let rating_agent: RatingAgentSender<WasmHost> =
        RatingAgentSender::to_actor(&format!("agent/{}", rating_request.agent_id.to_string()));

    info!("rating agent: {:?}", rating_agent);

    let validation_request = ValidationRequest {
        rating_request: rating_request.to_owned(),
        client_ip: client_ip,
        client_country: client_country,
    };

    let validation_response = rating_agent.validate(ctx, &validation_request).await?;

    Ok(validation_response)
}

async fn handle_rating_cycle(
    _ctx: &Context,
    rating_process_request: &RatingProcessRequest,
    rating_agents_stack: &mut Vec<(String, String)>
) -> RpcResult<RatingResponse> {
    let mut next_agent = rating_agents_stack.pop().unwrap();

    let mut updated_rating_request = rating_process_request.rating_request.clone();
    updated_rating_request.customer_id = next_agent.1;
    updated_rating_request.agent_id = next_agent.0;

    info!("Rating against agent: {}", updated_rating_request.agent_id);

    let mut rating_response =
        rate_through_agent(_ctx, &updated_rating_request).await?;

    info!("Vendor Rating status: {}", rating_response.authorization_status.code);
    info!("Vendor Rating have next agent and valid: {}",
        rating_response.authorization_status.code != 401 && !rating_agents_stack.is_empty());
    
    while rating_response.authorization_status.code != 401 && !rating_agents_stack.is_empty() {
        next_agent = rating_agents_stack.pop().unwrap();
        info!("Rating against provider agent: {}", next_agent.0);

        let next_agent_name = next_agent.0;
        let next_partner_id = next_agent.1;

        updated_rating_request.customer_id = next_partner_id;
        updated_rating_request.agent_id = next_agent_name;

        rating_response = rate_through_agent(_ctx, &updated_rating_request).await?;

        info!("Provider Rating status: {}", rating_response.authorization_status.code);
        info!("Provider Rating have next agent and valid: {}",
            rating_response.authorization_status.code != 401 && !rating_agents_stack.is_empty());
    }

    info!("final Rating response: {}", rating_response.authorization_status.code);

    Ok(rating_response)
}

async fn rate_through_agent(
    ctx: &Context,
    rating_request: &RatingRequest,
) -> RpcResult<RatingResponse> {
    let rating_agent: RatingAgentSender<WasmHost> =
        RatingAgentSender::to_actor(&format!("agent/{}", rating_request.agent_id.to_string()));

    let rating_response = rating_agent.rate_usage(ctx, rating_request).await?;

    Ok(rating_response)
}
