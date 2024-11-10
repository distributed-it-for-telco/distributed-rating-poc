

use crate::orange::rating::types::{
    AgentIdentification, AuthorizationStatus, BillingInformation,
};
use exports::orange::rating::ratingagent::*;
use wasi::logging::logging::log;
use std::fmt;
use serde::{Serialize, Deserialize};
use crate::orange::balancemanager::*;

wit_bindgen::generate!({
    generate_all,
    additional_derives: [serde::Serialize, serde::Deserialize]
});


struct VideoRatingagent;

impl Guest for VideoRatingagent {
    /// Say hello!
    fn rate_usage(_request: RatingRequest) -> RatingResponse {

        log(wasi::logging::logging::Level::Info, "", &"Hello I'm your video provider postpaid rating agent".to_string());

        let price: f32 = 1.0;
        let rating = _request.usage.usage_characteristic_list[0].value.parse::<f32>().unwrap() * price;

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
