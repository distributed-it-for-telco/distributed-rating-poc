use rating_interface::{
    AuthorizationStatus, BillingInformation, RatingAgent, RatingAgentReceiver, RatingAgentSender,
    RatingRequest, RatingResponse, UsageCollectorSender, UsageCollector
};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::info;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, RatingAgent)]
struct OrangeVodRatingAgentActor {}

/// Implementation of Rating Agent trait methods
#[async_trait]
impl RatingAgent for OrangeVodRatingAgentActor {
    async fn rate_usage(&self, _ctx: &Context, _arg: &RatingRequest) -> RpcResult<RatingResponse> {
        info!("Hello I'm your orange vod rating agent");
        let usage = "{\"usage\": 100}";

        let rating = UsageCollectorSender::to_actor(&format!("mock/{}", "usage_collector"))
            .store(_ctx, &usage)
            .await?;

        RpcResult::Ok(RatingResponse {
            authorization_status: AuthorizationStatus::default(),
            billing_information: BillingInformation::default(),
        })
    }
}
