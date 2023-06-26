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

        let validation_response_as_rating = handle_validation_cycle(_ctx, _arg).await?;

        if validation_response_as_rating.authorization_status.code == 401 {
            return RpcResult::Ok(validation_response_as_rating);
        }

        RpcResult::from(match handle_rating_cycle(_ctx, _arg).await {
            Ok(rating) => Ok(rating),
            Err(e) => Err(e),
        })
    }
}

async fn handle_validation_cycle(
    _ctx: &Context,
    rating_process_request: &RatingProcessRequest,
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
) -> RpcResult<RatingResponse> {
    info!("Rating against agent: {}", rating_process_request.rating_request.agent_id);

    let mut rating_response =
        rate_through_agent(_ctx, &rating_process_request.rating_request).await?;


    while rating_response.authorization_status.code != 401 && rating_response.next_agent != None {
        info!("Rating against agent: {}", rating_response.next_agent.to_owned().unwrap().name);

        let next_agent_name = rating_response.next_agent.to_owned().unwrap().name;
        let next_partner_id = rating_response.next_agent.to_owned().unwrap().partner_id;

        let mut updated_rating_request = rating_process_request.rating_request.clone();
        updated_rating_request.customer_id = next_partner_id;
        updated_rating_request.agent_id = next_agent_name;

        rating_response = rate_through_agent(_ctx, &updated_rating_request).await?;
    }

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
