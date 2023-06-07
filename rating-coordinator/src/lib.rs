use std::collections::HashMap;

use rating_interface::{
    RatingAgent, RatingAgentReceiver, RatingAgentSender, RatingRequest, RatingResponse,
};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::info;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor)]
struct RatingAgentCoordinatorActor {}

/// Implementation of Rating Coodinator
impl RatingAgentCoordinatorActor {
    async fn handle_rating_process(&self, _ctx: &Context, _arg: &RatingRequest, headers: HashMap<String, String>) -> RpcResult<RatingResponse> {
        info!("Hello I'm your rating coordinator");
        info!("Current used agent is: {}", _arg.agent_id);

        let rating_agent = RatingAgentSender::to_actor(&format!("agent/{}", _arg.agent_id));

        RpcResult::from(match rating_agent.rate_usage(_ctx, _arg).await {
            Ok(rating) => Ok(rating),
            Err(e) => Err(e),
        })
    }
}
