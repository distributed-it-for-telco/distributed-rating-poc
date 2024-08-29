wit_bindgen::generate!({
    generate_all
});

use exports::orange::ratingagent::ratingagent::*;
use crate::orange::ratingagent::types::{AgentIdentification, BillingInformation, AuthorizationStatus};

struct Ratingagent;

impl Guest for Ratingagent {
    /// Say hello!
    fn rate_usage(_request:RatingRequest) ->  RatingResponse{
        
        RatingResponse{
            authorization_status:AuthorizationStatus {code: 12345, key: "hello".to_string()},
            billing_information:BillingInformation {price:"myPriceAbc".to_string(), unit:"woow".to_string(), messages:(&["".to_string()]).to_vec()},
            next_agent:AgentIdentification{name:"hola".to_string(), partner_id:"amigos".to_string()}
        }
    }

    fn validate(_request: ValidationRequest) -> ValidationResponse {
        ValidationResponse{
            valid:true
        }
    }

    fn get_children(_request: GetChildrenRequest) -> AgentList {
        AgentList{
            agents: (&[]).to_vec()
        }
    }

}


export!(Ratingagent);
