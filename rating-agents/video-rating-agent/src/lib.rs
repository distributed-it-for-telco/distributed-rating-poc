use rating_interface::*;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::info;
use wasmcloud_interface_numbergen::generate_guid;

const OFFER_ID: &str = "video";
const RATING_PROOF_DESC: &str = "Video on Demand for Vendors";
const RATING_PROOF_USAGE_TYPE: &str = "VoDVend";
const RATING_PROOF_PRODUCT_NAME: &str = "Video on Demand for Vendors";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, RatingAgent)]
struct VideoRatingAgentActor {}

/// Implementation of Rating Agent trait methods
#[async_trait]
impl RatingAgent for VideoRatingAgentActor {
    async fn rate_usage(&self, _ctx: &Context, _arg: &RatingRequest) -> RpcResult<RatingResponse> {
        info!("Hello I'm your video provider postpaid rating agent");

        let usage_date = "21/06/2023";
        let usage_id: String = generate_guid().await?;

        /*
         *  Contract or Offer is one Movie equal one EURO
         */
        info!("_arg.usage.usage_characteristic_list {}",_arg.usage.usage_characteristic_list.len());
        let mut rating=0;
         if let Some(first) = _arg.usage.usage_characteristic_list.first() {
            rating = first.value.parse::<i32>().unwrap()*1;
        }

        let usage_template_str = UsageProofHandler::generate_rating_proof(&UsageProofRequest {
            party_id: _arg.customer_id.to_owned(),
            rating: rating.to_string(),
            usage_characteristic_list: _arg.usage.usage_characteristic_list.to_owned(),
            usage_id: usage_id.as_str().to_owned(),
            usage_date: usage_date.to_owned(),
            offer_id: OFFER_ID.to_owned(),
            description: RATING_PROOF_DESC.to_owned(),
            usage_type: RATING_PROOF_USAGE_TYPE.to_owned(),
            product_name: RATING_PROOF_PRODUCT_NAME.to_owned(),
        });

        info!(
            "Sending usage proof to video provider usage collector for party with id: {}",
            _arg.customer_id.to_string()
        );

        UsageCollectorSender::to_actor(&format!("mock/{}", "usage_collector_video_provider"))
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
        let mut validation_response: ValidationResponse = ValidationResponse::default();
        validation_response.next_agent = None;

        if arg.client_country.is_some() && arg.client_country.to_owned().unwrap() == "EG" {
            validation_response.valid = true;
        } else {
            validation_response.valid = false;
        }

        Ok(validation_response)
    }
}
