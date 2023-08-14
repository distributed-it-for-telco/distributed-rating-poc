use rating_interface::{
    AgentIdentifiation, RatingAgent, RatingAgentReceiver, RatingRequest, RatingResponse,
    RatingResponseBuilder, UsageCollector, UsageCollectorSender, UsageProofHandler,
    UsageProofRequest, ValidationRequest, ValidationResponse, UsageCharacteristic,
};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::info;
use wasmcloud_interface_numbergen::generate_guid;

const OFFER_ID: &str = "100";
const DROPBOX_PARTY_ID_AT_PARTNER_SIDE: &str = "dropbox_my_partner";
const PROVIDER_AGENT_NAME: &str = "aws_stor";
const REPLICATION_FACTOR: i32 = 2;
const RATE_FEE: i32 = 1;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, RatingAgent)]
struct DropboxSyncstorRatingAgentActor {}

/// Implementation of the HttpServer capability contract
#[async_trait]
impl RatingAgent for DropboxSyncstorRatingAgentActor {
    async fn rate_usage(&self, _ctx: &Context, _arg: &RatingRequest) -> RpcResult<RatingResponse> {

        if _arg.usage.usage_characteristic_list.is_empty() {
            return RpcResult::from(Err(RpcError::Other(
                "Can't rate usage, no characteristic sent!".to_owned(),
            )));
        }

        info!("Hello I'm your Dropbox Syncstor postpaid rating agent");

        let usage_date = "07/08/2023";
        let usage_id: String = generate_guid().await?;

        /*
         *  Contract or Offer is 1 GB = 1 EUR 
         */

        let mut rating = RATE_FEE;

        if let Some(first) = _arg.usage.usage_characteristic_list.first() {
            rating *= first.value.parse::<i32>().unwrap();
        }
        
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

        UsageCollectorSender::to_actor(&format!("mock/{}", "usage_collector_dropbox"))
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
        next_agent.partner_id = DROPBOX_PARTY_ID_AT_PARTNER_SIDE.to_string();

        validation_response.next_agent = Some(next_agent);

        let mut translated_usage = arg.rating_request.usage.to_owned();
        let replica_count_usage = UsageCharacteristic {
            name: "replica-count".to_string(),
            value: REPLICATION_FACTOR.to_string(),
            value_type: "integer".to_string()
        };
        translated_usage.usage_characteristic_list.push(replica_count_usage);

        validation_response.translated_usage = Some(translated_usage);
        Ok(validation_response)
    }
}