use rating_interface::{
    AgentIdentifiation, RatingAgent, RatingAgentReceiver, RatingRequest, RatingResponse,
    RatingResponseBuilder, UsageCollector, UsageCollectorSender, UsageProofHandler,
    UsageProofRequest, ValidationRequest, ValidationResponse, UsageCharacteristic,
};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::info;
use wasmcloud_interface_numbergen::generate_guid;

const OFFER_ID: &str = "1000";
const AWS_PARTY_ID_AT_PARTNER_SIDE: &str = "aws_my_partner";
const PROVIDER_AGENT_NAME: &str = "orange_connectivity";
const REPLICATION_FACTOR: u32 = 2;
const RATE_FEE: f32 = 1.0;
#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, RatingAgent)]
struct AwsStorRatingAgentActor {}

#[async_trait]
impl RatingAgent for AwsStorRatingAgentActor {
    async fn rate_usage(&self, _ctx: &Context, _arg: &RatingRequest) -> RpcResult<RatingResponse> {
        if _arg.usage.usage_characteristic_list.is_empty() {
            return RpcResult::from(Err(RpcError::Other(
                "Can't rate usage, no characteristic sent!".to_owned(),
            )));
        }

        info!("Hello I'm your AWS stor postpaid rating agent");

        let usage_date = "07/08/2023";
        let usage_id: String = generate_guid().await?;

        /*
         *  Contract or Offer is 1 GB = 1 EUR
         */

        let mut storage_usage: f32 = 1.0;
        for characteristic in _arg.usage.usage_characteristic_list.clone().iter_mut() {
            storage_usage *= characteristic.value.parse::<f32>().unwrap();
        }

        let rating = RATE_FEE * storage_usage;

        let usage_template_str = UsageProofHandler::generate_rating_proof(&UsageProofRequest {
            party_id: _arg.customer_id.to_owned(),
            rating: rating.to_string(),
            usage_characteristic_list: _arg.usage.usage_characteristic_list.to_owned(),
            usage_id: usage_id.as_str().to_owned(),
            usage_date: usage_date.to_owned(),
            offer_id: OFFER_ID.to_owned(),
        });

        info!(
            "Sending usage proof to usage collector for party with id: {}",
            _arg.customer_id.to_string()
        );

        UsageCollectorSender::to_actor(&format!("mock/{}", "usage_collector_aws"))
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

    /// Validate
    /// 1- apply business validation.
    /// 2- translate usage.
    /// 3- return validation status and tranlated usage.
    async fn validate(
        &self,
        ctx: &Context,
        arg: &ValidationRequest,
    ) -> RpcResult<ValidationResponse> {
        let mut validation_response: ValidationResponse = ValidationResponse::default();

        if arg.client_country.is_some() && arg.client_country.to_owned().unwrap() == "EG" {
            validation_response.valid = true;
        } else {
            validation_response.valid = false;
        }

        let mut next_agent: AgentIdentifiation = AgentIdentifiation::default();

        next_agent.name = PROVIDER_AGENT_NAME.to_string();
        next_agent.partner_id = AWS_PARTY_ID_AT_PARTNER_SIDE.to_string();

        validation_response.next_agent = Some(next_agent);

        let mut connectivity: f32 = 1.0;
        for  characteristic in arg.rating_request.usage.usage_characteristic_list.to_owned().iter_mut() {
            connectivity *= characteristic.value.parse::<f32>().unwrap();
        }

        let mut translated_usage = arg.rating_request.usage.to_owned();
        let connectivity_usage = UsageCharacteristic {
            name: "connectivity".to_string(),
            value: connectivity.to_string(),
            value_type: "float".to_string(),
        };
        let mut characteristicsVector = vec![connectivity_usage];

        translated_usage.usage_characteristic_list = characteristicsVector;

        validation_response.translated_usage = Some(translated_usage);
        Ok(validation_response)
    }

}