use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::{info};
use rating_interface::{RatingAgent, RatingRequest, RatingResponse, RatingAgentSender, RatingAgentReceiver};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, RatingAgent)]
struct RatingAgentCoordinatorActor {}

const KNOWN_AGENTS_NAMES: &[&str] = &["orange"];

/// Implementation of Rating Agent trait methods
#[async_trait]
impl RatingAgent for RatingAgentCoordinatorActor {
    async fn rate_usage(&self, _ctx: &Context, _arg: &RatingRequest) -> RpcResult<RatingResponse> {
        // should call different rating agents here
        let agents = KNOWN_AGENTS_NAMES.to_vec();
        info!("Hello I'm your rating coordinator");

        // let mut ratings = Vec::new();
        // for agent in agents {
            let rating_agent = RatingAgentSender::to_actor(&format!("agent/{}", "orange"));
            let rating = rating_agent
                .rate_usage(_ctx, _arg)
                .await.expect("msg");

        //    ratings.push(rating.clone());
        // }

        RpcResult::Ok(rating)

        // RpcResult::Ok(
        //     RatingResponse { 
        //         authorization_status: AuthorizationStatus::default(), 
        //         billing_information: BillingInformation::default()
        //     }
        // )

        // RpcResult::Err(RpcError::NotImplemented)
    }

}

