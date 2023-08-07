use rating_interface::{
    RatingAgent, RatingAgentSender,
    RatingProcessRequest, RatingResponse,
    Usage, RatingRequest, RatingRecord,
};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::info;

pub async fn handle_rating_cycle(
    _ctx: &Context,
    rating_process_request: &RatingProcessRequest,
    rating_agents_stack: &mut Vec<(String, String, Option<Usage>)>,
) -> RpcResult<RatingResponse> {

    let mut updated_rating_request = rating_process_request.rating_request.clone();
    if let Some(next_agent) = rating_agents_stack.pop(){
        updated_rating_request.agent_id = next_agent.0;
        updated_rating_request.customer_id = next_agent.1;
        if let Some(usage) = next_agent.2 {
            updated_rating_request.usage = usage;
        }
    }
    
    info!("Rating against agent: {}", updated_rating_request.agent_id);

    let mut rating_response = rate_through_agent(_ctx, &updated_rating_request).await?;

    info!(
        "Vendor Rating status: {}",
        rating_response.authorization_status.code
    );
    info!(
        "Vendor Rating have next agent and valid: {}",
        rating_response.authorization_status.code != 401 && !rating_agents_stack.is_empty()
    );

    while rating_response.authorization_status.code != 401 && !rating_agents_stack.is_empty() {
        info!("Prepare rating history .....");
        let mut rating_record: RatingRecord = RatingRecord::default();
        rating_record.producer = updated_rating_request.agent_id.to_owned();
        rating_record.unit = rating_response.billing_information.unit;
        rating_record.price = rating_response.billing_information.price;
        let mut rating_history: Vec<RatingRecord> = Vec::new();
        let rec = rating_record.clone();
        rating_history.push(rating_record);

        info!(
            "Rating History {:?},{:?},{:?}",
            rec.producer, rec.unit, rec.price
        );

        if let Some(next_agent) = rating_agents_stack.pop(){
            info!("Rating against provider agent: {}", next_agent.0);

            updated_rating_request.agent_id = next_agent.0;
            updated_rating_request.customer_id = next_agent.1;
            if let Some(usage) = next_agent.2 {
                updated_rating_request.usage = usage;
            }
        }

        updated_rating_request.rating_history = Some(rating_history);

        rating_response = rate_through_agent(_ctx, &updated_rating_request).await?;

        info!(
            "Provider Rating status: {}",
            rating_response.authorization_status.code
        );
        info!(
            "Provider Rating have next agent and valid: {}",
            rating_response.authorization_status.code != 401 && !rating_agents_stack.is_empty()
        );
    }

    info!(
        "final Rating response: {}",
        rating_response.authorization_status.code
    );

    Ok(rating_response)
}

pub async fn rate_through_agent(
    ctx: &Context,
    rating_request: &RatingRequest,
) -> RpcResult<RatingResponse> {
    let rating_agent: RatingAgentSender<WasmHost> =
        RatingAgentSender::to_actor(&format!("agent/{}", rating_request.agent_id.to_string()));

    let rating_response = rating_agent.rate_usage(ctx, rating_request).await?;

    Ok(rating_response)
}
