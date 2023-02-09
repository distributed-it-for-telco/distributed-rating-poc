use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::{info};
use rating_interface::{RatingAgent, RatingRequest, RatingResponse, RatingAgentSender, RatingAgentReceiver, AuthorizationStatus, BillingInformation};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, RatingAgent)]
struct OrangeRatingAgentActor {}

/// Implementation of Rating Agent trait methods
#[async_trait]
impl RatingAgent for OrangeRatingAgentActor {
    async fn rate_usage(&self, _ctx: &Context, _arg: &RatingRequest) -> RpcResult<RatingResponse> {
        info!("Hello I'm your orange rating agent");

        RpcResult::Ok(
            RatingResponse { 
                authorization_status: AuthorizationStatus::default(), 
                billing_information: BillingInformation::default()
            }
        )
    }

}

