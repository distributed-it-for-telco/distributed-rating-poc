wit_bindgen::generate!({
    generate_all
});

use std::collections::HashMap;
use lazy_static::lazy_static;
use futures::executor::block_on;
use wasi::logging::logging::{log,Level::Info};
use crate::orange::commons::commons::generate_rating_proof;
use crate::orange::commons::types::{RatingResponse, ValidationRequest, UsageProofRequest,
                                    AgentIdentification, Agent};
use crate::orange::commons::error_types::{UsageError, ValidationError};
use crate::orange::commons::rating_response_builder as RatingResponseBuilder;
use crate::orange::usagecollector::usagecollector;
use exports::orange::rating::ratingagent::*;

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

struct OrangeVodRatingagent;

impl OrangeVodRatingagent {
    async fn rate_usage_async(request: RatingRequest) -> Result<RatingResponse, UsageError> {
        log(Info,"","Hello I'm your orange postpaid vod rating agent");
        let usage_date = "04/04/2023";
        let usage_id: String = "".to_string();// generate_guid().await?;
        
        /*
         *  Contract or Offer is 10% added to provider price
         */


        // let previouse_rating_price_str = request.rating_history.clone().pop().unwrap().price;
        let previouse_rating_price = 12.5;//previouse_rating_price_str.parse::<f64>().unwrap();
        let rating = previouse_rating_price + (previouse_rating_price * RATE_FEE);

        let usage_template_str = generate_rating_proof(&UsageProofRequest {
            party_id: request.customer_id.to_owned(),
            rating: rating.to_string(),
            usage_characteristic_list: request.usage.usage_characteristic_list.to_owned(),
            usage_id: usage_id.as_str().to_owned(),
            usage_date: usage_date.to_owned(),
            offer_id: OFFER_ID.to_owned(),
            description: RATING_PROOF_DESC.to_owned(),
            usage_type: RATING_PROOF_USAGE_TYPE.to_owned(),
            product_name: RATING_PROOF_PRODUCT_NAME.to_owned(),
        });

        log(Info,"",
            format!("Sending usage proof to usage collector for party with id: {}",
            request.customer_id).as_str()
        );
        usagecollector::store(&usage_template_str);

        let mut response_builder_handle = RatingResponseBuilder::create_builder();
        response_builder_handle= RatingResponseBuilder::unit(response_builder_handle, &"EUR");
        response_builder_handle= RatingResponseBuilder::price(response_builder_handle, &rating.to_string());
        response_builder_handle= RatingResponseBuilder::message(response_builder_handle, &rating.to_string());
        response_builder_handle= RatingResponseBuilder::message(response_builder_handle, &rating.to_string());
        response_builder_handle= RatingResponseBuilder::authorized(response_builder_handle);
        Ok(RatingResponseBuilder::build(response_builder_handle))
    }

    async fn validate_async(request: ValidationRequest) -> Result<ValidationResponse, ValidationError>  {
        let mut validation_response: ValidationResponse = ValidationResponse{
            valid: true
        };
        if !request.client_country.is_empty() && request.client_country.to_owned() == "EG" {
            validation_response.valid = true;
        } else {
            validation_response.valid = false;
        }
        Ok(validation_response)
    }

    async fn get_children_async(request: GetChildrenRequest) -> AgentList {
        let mut children_list = AgentList{
            agents: vec![]
        };

        if !request.atomic_offer_id.is_empty()
            && OFFER_PROVIDERS_OFFERS_IDS_TO_AGENTS
                .contains_key(request.atomic_offer_id.to_owned().as_str())
        {
            let child = Agent {
                identification: AgentIdentification {
                    name: OFFER_PROVIDERS_OFFERS_IDS_TO_AGENTS
                        .get(request.atomic_offer_id.to_owned().as_str())
                        .unwrap()
                        .to_string(),
                    partner_id: ORANGE_PARTY_ID_AT_PARTNER_SIDE.to_string(),
                },
                usage: request.usage.clone(),
            };
            children_list.agents.push(child);
        }
        children_list
    }
}

impl Guest for OrangeVodRatingagent {
    fn rate_usage(request: RatingRequest) -> Result<RatingResponse, UsageError>  {
        block_on(Self::rate_usage_async(request))
    }
    fn validate(request: ValidationRequest) -> Result<ValidationResponse, ValidationError> {
        block_on(Self::validate_async(request))
    }
    fn get_children(request: GetChildrenRequest) -> AgentList {
        block_on(Self::get_children_async(request))
    }
}

export!(OrangeVodRatingagent);
