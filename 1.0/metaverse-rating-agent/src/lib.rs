
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


struct MetaverseRatingagent;

impl Guest for MetaverseRatingagent {
    /// Say hello!
    fn rate_usage(_request: RatingRequest) -> RatingResponse {

        let balance = balancemanager::get_balance(&_request.customer_id, &_request.offer_id);
        log(wasi::logging::logging::Level::Info, "", &balance.count.to_string());

        let price: f32 = 5.0;
        let purchase_amount = _request.usage.usage_characteristic_list[0].value.parse::<f32>().unwrap() * price;

        // Attempt to purchase
        let message: String;

        match balancemanager::purchase(&balance, purchase_amount, &_request.customer_id, &_request.offer_id) {
            Ok(updated_balance) => {
                message = "Purchase successful".to_string();
            },
            Err(err) => {
                message = "Purchase failed: ".to_string() + &err;
            },
        }

        RatingResponse {
            authorization_status: AuthorizationStatus {
                code: 12345,
                key: "hello".to_string(),
            },
            billing_information: BillingInformation {
                price: price.to_string(),
                unit: balance.unit.to_string(),
                messages: vec![message],
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

export!(MetaverseRatingagent);
