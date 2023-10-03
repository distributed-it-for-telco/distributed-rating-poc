use lazy_static::lazy_static;
use std::collections::HashMap;

use rating_interface::{
    AgentIdentifiation, RatingAgent, RatingAgentReceiver, RatingRequest, RatingResponse,
    RatingResponseBuilder, UsageCollector, UsageCollectorSender, UsageProofHandler,
    UsageProofRequest, ValidationRequest, ValidationResponse, GetChildrenRequest, AgentList, Agent,
};

use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::info;
use wasmcloud_interface_numbergen::generate_guid;

const OFFER_ID: &str = "1";
const ORANGE_PARTY_ID_AT_PARTNER_SIDE: &str = "orange_my_partner";
const RATE_FEE: f64 = 0.1;
const RATING_PROOF_DESC: &str = "Streamzie Movies on demand";
const RATING_PROOF_USAGE_TYPE: &str = "VoD";
const RATING_PROOF_PRODUCT_NAME: &str = "Streamzie Movies on demand";

lazy_static! {
    static ref OFFER_PROVIDERS_OFFERS_IDS_TO_AGENTS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("stream", "provider_streaming");
        m.insert("video", "provider_video");
        m
    };
}

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
         *  Contract or Offer is 50% added to provider price
         */

        let previouse_rating_price_str = _arg.rating_history.clone().unwrap().pop().unwrap().price;
        let previouse_rating_price = previouse_rating_price_str.parse::<f64>().unwrap();
        let rating = previouse_rating_price + (previouse_rating_price * RATE_FEE);

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

        UsageCollectorSender::to_actor(&format!("mock/{}", "usage_collector_orange"))
            .store(_ctx, &usage_template_str)
            .await?;

        let mut rating_response_builder = RatingResponseBuilder::new();

        let rating_response = rating_response_builder
            .unit((&"EUR").to_string())
            .price(rating.to_string())
            .message(&format!("You can now enjoy your movie on Streamzie"))
            .message(&format!(
                "The cost of this transaction is {} EUR",
                rating.to_string()
            ))
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

        if arg.atomic_offer_id.is_some()
            && OFFER_PROVIDERS_OFFERS_IDS_TO_AGENTS
                .contains_key(arg.atomic_offer_id.to_owned().unwrap().as_str())
        {
            let child = Agent {
                identifiation: AgentIdentifiation {
                    name: OFFER_PROVIDERS_OFFERS_IDS_TO_AGENTS
                    .get(arg.atomic_offer_id.to_owned().unwrap().as_str())
                    .unwrap()
                    .to_string(),
                    partner_id:  ORANGE_PARTY_ID_AT_PARTNER_SIDE.to_string(),
                },
                usage: Some(arg.usage.clone()),
            };
            children_list.push(child);
        }

        Ok(children_list)
    }
}
