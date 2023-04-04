use rating_interface::{
    RatingAgent, RatingAgentReceiver, RatingAgentSender, RatingRequest, RatingResponse,
};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::info;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, RatingAgent)]
struct RatingAgentCoordinatorActor {}

/// Implementation of Rating Agent trait methods
#[async_trait]
impl RatingAgent for RatingAgentCoordinatorActor {
    async fn rate_usage(&self, _ctx: &Context, _arg: &RatingRequest) -> RpcResult<RatingResponse> {
        info!("Hello I'm your rating coordinator");
        info!("Current used agent is: {}", _arg.agent_id);

        let rating_agent = RatingAgentSender::to_actor(&format!("agent/{}", _arg.agent_id));

        RpcResult::from(match rating_agent.rate_usage(_ctx, _arg).await {
            Ok(rating) => Ok(rating),
            Err(e) => Err(e),
        })
    }
}
