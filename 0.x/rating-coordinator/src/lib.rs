mod agent_graph;
mod build_agents_hierarchy;
mod rating;
mod validation;

use build_agents_hierarchy::*;
use rating::*;
use validation::*;

use rating_interface::{
    RatingCoordinator, RatingCoordinatorReceiver, RatingProcessRequest, RatingResponse,
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

        let agent_graph = build_agent_hierarchy(&_ctx, &_arg.rating_request).await?;

        info!("Graph generated ......");

        let validation_response_as_rating =
            match handle_validation_cycle(_ctx, _arg, &agent_graph).await {
                Ok(validation) => validation,
                Err(e) => return RpcResult::from(Err(e)),
            };

        if validation_response_as_rating.authorization_status.code == 401 {
            return RpcResult::Ok(validation_response_as_rating);
        }

        RpcResult::from(match handle_rating_cycle(_ctx, _arg, &agent_graph).await {
            Ok(rating) => Ok(rating),
            Err(e) => Err(e),
        })
    }
}
