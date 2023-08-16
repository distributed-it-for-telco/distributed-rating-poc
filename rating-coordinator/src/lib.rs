mod rating;
mod validation;
use validation::*;
use rating::*;

use rating_interface::{
    RatingCoordinator, RatingCoordinatorReceiver,
    RatingProcessRequest, RatingResponse, Usage
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

        let mut rating_agents_stack: Vec<(String, String, Option<Usage>)> = Vec::new();

        let validation_response_as_rating =
            match handle_validation_cycle(_ctx, _arg, &mut rating_agents_stack).await {
                Ok(validation) => validation,
                Err(e) => return RpcResult::from(Err(e)),
            };

        if validation_response_as_rating.authorization_status.code == 401 {
            return RpcResult::Ok(validation_response_as_rating);
        }

        RpcResult::from(
            match handle_rating_cycle(_ctx, _arg, &mut rating_agents_stack).await {
                Ok(rating) => Ok(rating),
                Err(e) => Err(e),
            },
        )
    }
}