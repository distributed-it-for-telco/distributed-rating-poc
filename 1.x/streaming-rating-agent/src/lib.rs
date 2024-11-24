
use crate::orange::rating::types::{
    AgentIdentification, AuthorizationStatus, BillingInformation,
};
use exports::orange::rating::ratingagent::*;
use wasi::logging::logging::log;

wit_bindgen::generate!({
    generate_all,
    additional_derives: [serde::Serialize, serde::Deserialize]
});
struct StreamingRatingagent;

impl Guest for StreamingRatingagent {

    fn rate_usage(_request: RatingRequest) -> RatingResponse {
        log(wasi::logging::logging::Level::Info, "", &"Streaming Rating Agent".to_string());

        RatingResponse {
            authorization_status: AuthorizationStatus {
                code: 12345,
                key: "two".to_string(),
            },
            billing_information: BillingInformation {
                price: "price".to_string(),
                unit: "unit".to_string(),
                messages: vec!["message".to_string()],
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

export!(StreamingRatingagent);
