use rating_interface::{
    AuthorizationStatus, BillingInformation, RatingAgent, RatingAgentReceiver, RatingRequest,
    RatingResponse, RatingResponseBuilder, UsageCollector, UsageCollectorSender, UsageProofHandler,
    UsageProofRequest, ValidationRequest, ValidationResponse,
};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::info;
use wasmcloud_interface_numbergen::generate_guid;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, RatingAgent)]
struct OrangeVodRatingAgentActor {}

/// Implementation of Rating Agent trait methods
#[async_trait]
impl RatingAgent for OrangeVodRatingAgentActor {
    async fn rate_usage(&self, _ctx: &Context, _arg: &RatingRequest) -> RpcResult<RatingResponse> {
        info!("Hello I'm your orange postpaid vod rating agent");

        let usage_date = "04/04/2023";
        let usage_id: String = generate_guid().await?;

        /*
         *  Contract or Offer is one Movie equal one EURO
         */
        let rating = _arg.usage.parse::<i32>().unwrap() * 1;

        let usage_template_str = UsageProofHandler::generate_rating_proof(&UsageProofRequest {
            party_id: _arg.customer_id.to_owned(),
            rating: rating.to_string(),
            usage: _arg.usage.to_owned(),
            usage_id: usage_id.as_str().to_owned(),
            usage_date: usage_date.to_owned(),
        });

        info!(
            "Sending usage proof to usage collector for party with id: {}",
            _arg.customer_id.to_string()
        );

        UsageCollectorSender::to_actor(&format!("mock/{}", "usage_collector"))
            .store(_ctx, &usage_template_str)
            .await?;

        let mut rating_response_builder = RatingResponseBuilder::new();

        let rating_response = rating_response_builder
            .unit((&"EUR").to_string())
            .price(rating.to_string())
            .authorized()
            .build();

        RpcResult::Ok(rating_response)
    }

    async fn validate(
        &self,
        ctx: &Context,
        arg: &ValidationRequest,
    ) -> RpcResult<ValidationResponse> {
        todo!()
    }
}
