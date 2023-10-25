use lazy_static::lazy_static;
use rating_interface::{
    Agent, AgentIdentifiation, AgentList, GetChildrenRequest, RatingAgent, RatingAgentReceiver,
    RatingRequest, RatingResponse, RatingResponseBuilder, Usage, UsageCharacteristic,
    UsageCollector, UsageCollectorSender, UsageProofHandler, UsageProofRequest, ValidationRequest,
    ValidationResponse,
};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::info;
use wasmcloud_interface_numbergen::generate_guid;

const OFFER_ID: &str = "100";
const DROPBOX_PARTY_ID_AT_PARTNER_SIDE: &str = "dropbox_composit_horizontal_my_partner";
//const PROVIDER_AGENTS_NAMES:Vec<&str> = vec!["aws_stor", "orange_connectivity"];

lazy_static! {
    static ref PROVIDER_AGENTS_NAMES: Vec<&'static str> = {
        let mut v = Vec::new();
        v.push("aws_stor");
        v.push("orange_connectivity");
        v
    };
}

const REPLICATION_FACTOR: i32 = 2;
const RATE_FEE: i32 = 1;
const RATING_PROOF_DESC: &str = "Dropbox Syncstor Composite Horizontal";
const RATING_PROOF_USAGE_TYPE: &str = "DSS";
const RATING_PROOF_PRODUCT_NAME: &str = "Dropbox Syncstor Composite Horizontal";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, RatingAgent)]
struct DropboxSyncstorCompositeHorizontalRatingAgentActor {}

/// Implementation of the HttpServer capability contract
#[async_trait]
impl RatingAgent for DropboxSyncstorCompositeHorizontalRatingAgentActor {
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
            description: RATING_PROOF_DESC.to_owned(),
            usage_type: RATING_PROOF_USAGE_TYPE.to_owned(),
            product_name: RATING_PROOF_PRODUCT_NAME.to_owned(),
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

        Ok(validation_response)
    }

    async fn get_children(&self, ctx: &Context, arg: &GetChildrenRequest) -> RpcResult<AgentList> {
        let mut children_list = AgentList::new();

        let replica_count_usage = UsageCharacteristic {
            name: "replica-count".to_string(),
            value: REPLICATION_FACTOR.to_string(),
            value_type: "integer".to_string(),
        };

        let mut translated_usage = arg.usage.to_owned();
        translated_usage
            .usage_characteristic_list
            .push(replica_count_usage);

        let child = Agent {
            identifiation: AgentIdentifiation {
                name: PROVIDER_AGENTS_NAMES.get(0).unwrap().to_string(),
                partner_id: DROPBOX_PARTY_ID_AT_PARTNER_SIDE.to_string(),
            },
            usage: Some(translated_usage),
        };

        children_list.push(child);

        let mut connectivity: f32 = 1.0;
        for characteristic in arg.usage.usage_characteristic_list.to_owned().iter_mut() {
            connectivity *= characteristic.value.parse::<f32>().unwrap();
        }

        let connectivity_usage = UsageCharacteristic {
            name: "connectivity".to_string(),
            value: connectivity.to_string(),
            value_type: "float".to_string(),
        };

        let mut translated_usage = arg.usage.to_owned();
        let characteristics_vector = vec![connectivity_usage];
        translated_usage.usage_characteristic_list = characteristics_vector;

        let child = Agent {
            identifiation: AgentIdentifiation {
                name: PROVIDER_AGENTS_NAMES.get(1).unwrap().to_string(),
                partner_id: DROPBOX_PARTY_ID_AT_PARTNER_SIDE.to_string(),
            },
            usage: Some(translated_usage),
        };

        children_list.push(child);

        Ok(children_list)
    }
}
