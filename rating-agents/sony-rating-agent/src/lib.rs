use rating_interface::*;
use wasmbus_rpc::actor::prelude::*;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, RatingAgent)]
struct SonyRatingAgentActor {}

/// Implementation of Rating Agent trait methods
#[async_trait]
impl RatingAgent for SonyRatingAgentActor {
    async fn rate_usage(&self, _ctx: &Context, _arg: &RatingRequest) -> RpcResult<RatingResponse> {
        todo!()
    }
}
