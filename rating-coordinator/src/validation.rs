use rating_interface::{
    RatingAgent, RatingAgentSender,
    RatingProcessRequest, RatingResponse, RatingResponseBuilder,
    ValidationRequest, ValidationResponse, Usage, RatingRequest,
};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::info;

pub async fn handle_validation_cycle(
    _ctx: &Context,
    rating_process_request: &RatingProcessRequest,
    rating_agents_stack: &mut Vec<(String, String, Option<Usage>)>,
) -> RpcResult<RatingResponse> {
    if rating_process_request.headers.is_none() {
        return RpcResult::from(Err(RpcError::Other(
            "Can't validate client usage, client ip not found!".to_owned(),
        )));
    }

    let client_headers = rating_process_request.headers.to_owned().unwrap();
    let mut client_ip = "";
    let mut client_country = None;
    if let Some(extracted_client_ip) = client_headers.get("client_ip") {
        client_ip = extracted_client_ip;
    }

    if let Some(lcl_client_country) = client_headers.get("client_country") {
        client_country = Some(lcl_client_country);
    }

    info!(
        "Validating against agent: {}",
        rating_process_request.rating_request.agent_id
    );

    // Push main agent in the stack
    // validate throug main agent
    // push translated usage in usage stack
    rating_agents_stack.push((
        rating_process_request.rating_request.agent_id.to_owned(),
        rating_process_request.rating_request.customer_id.to_owned(),
        Some(rating_process_request.rating_request.usage.to_owned()),
    ));

    let mut validation_response: ValidationResponse = validate_through_agent(
        _ctx,
        &rating_process_request.rating_request,
        client_ip.to_string(),
        client_country.to_owned().cloned(),
    )
    .await?;
    
    info!("Vendor validation status: {}", validation_response.valid);
    info!(
        "Vendor validation have next agent and valid: {}",
        validation_response.valid && validation_response.next_agent != None
    );

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

        info!(
            "Validating against agent: {}",
            validation_response.next_agent.to_owned().unwrap().name
        );

        rating_agents_stack.push((
            updated_rating_request.agent_id.to_owned(),
            updated_rating_request.customer_id.to_owned(),
            validation_response.translated_usage.to_owned(),
        ));

        // updating usage in the request with the usage of current agent 
        // for the next to understand as next agent don't understand nothing other than the above level
        if let Some(translated_usage) = validation_response.translated_usage.to_owned() {
            updated_rating_request.usage = translated_usage.to_owned();
        }

        validation_response = validate_through_agent(
            _ctx,
            &updated_rating_request,
            client_ip.to_string(),
            client_country.to_owned().cloned(),
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

pub async fn validate_through_agent(
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
