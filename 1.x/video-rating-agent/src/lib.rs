

use crate::orange::rating::types::{
    AgentIdentification, AuthorizationStatus, BillingInformation,
};
use exports::orange::rating::ratingagent::*;
use wasi::logging::logging::log;
use std::fmt;
use serde::{Serialize, Deserialize};
use crate::orange::balancemanager::*;
use crate::orange::usagecollector::usagecollector;


wit_bindgen::generate!({
    generate_all,
    additional_derives: [serde::Serialize, serde::Deserialize]
});

const OFFER_ID: &str = "video";
const RATING_PROOF_DESC: &str = "Streamzie Movies on demand";
const RATING_PROOF_USAGE_TYPE: &str = "VoDVend";
const RATING_PROOF_PRODUCT_NAME: &str = "Streamzie Movies on demand";

struct VideoRatingagent;

impl Guest for VideoRatingagent {
    /// Say hello!
    fn rate_usage(_request: RatingRequest) -> RatingResponse {

        log(wasi::logging::logging::Level::Info, "", &"Hello I'm your video provider postpaid rating agent".to_string());

        let usage_date = "21/06/2023";
        let usage_id: String = "UUID".to_string();
        let rating_date = "04/04/2023";
        let price: f32 = 1.0;
        let rating = &_request.usage.usage_characteristic_list[0].value.parse::<f32>().unwrap() * price;

        let usage_template_str = serde_json::json!({
            "id": usage_id,
            "usageDate": usage_date,
            "description": RATING_PROOF_DESC,
            "usageType": RATING_PROOF_USAGE_TYPE,
            "ratedProductUsage": {
                "isBilled": false,
                "ratingAmountType": "Total",
                "ratingDate": rating_date,
                "bucketValueConvertedInAmount": {
                    "unit": "EUR",
                    "value": rating.to_string()
                },
                "productRef": {
                    "id": OFFER_ID,
                    "name": RATING_PROOF_PRODUCT_NAME
                }
            },
            "relatedParty": {
                "id": &_request.customer_id
            },
            "usageCharacteristic": &_request.usage.usage_characteristic_list
        }).to_string();

        log(wasi::logging::logging::Level::Info, "", &"Sending usage proof to video provider usage collector".to_string());

        usagecollector::store(&usage_template_str);

        log(wasi::logging::logging::Level::Info, "", &"Retrieving usage list from video provider usage collector\n\n\n".to_string());
        usagecollector::get_list().iter().for_each(|usage| {
            log(wasi::logging::logging::Level::Info, "", &usage.value.to_string());
        });

        // Attempt to purchase
        let message: String;

        RatingResponse {
            authorization_status: AuthorizationStatus {
                code: 12345,
                key: "hello".to_string(),
            },
            billing_information: BillingInformation {
                unit: (&"EUR").to_string(),
                price: rating.to_string(),
                messages: vec!["".to_string()],
            },
            next_agent: AgentIdentification {
                name: "agent".to_string(),
                partner_id: "partner".to_string(),
            },
        }
    }

    fn validate(_request: ValidationRequest) -> ValidationResponse {
        ValidationResponse { valid: true }
    }

    fn get_children(_request: GetChildrenRequest) -> AgentList {
        AgentList {
            agents: vec![],
        }
    }
}

export!(VideoRatingagent);
