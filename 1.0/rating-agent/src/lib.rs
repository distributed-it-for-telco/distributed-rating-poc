use exports::orange::ratingagent::ratingagent::*;
use crate::orange::ratingagent::types::{AgentIdentification, BillingInformation, AuthorizationStatus};
wit_bindgen::generate!({
    generate_all,
    additional_derives: [serde::Serialize, serde::Deserialize]
});


struct Ratingagent;

impl Guest for Ratingagent {
    /// Say hello!
    fn rate_usage(_request:RatingRequest) ->  RatingResponse{
        wasi::logging::logging::log( 
            wasi::logging::logging::Level::Info,
            "",&_request.offer_id
        );
        let bucket = wasi::keyvalue::store::open("").expect("failed to open empty bucket");
        let object_name = "count";
        let count = wasi::keyvalue::atomics::increment(&bucket, &object_name, 1)
            .expect("failed to increment count");
            
        RatingResponse{
            authorization_status:AuthorizationStatus {code: 12345, key: "hello".to_string()},
            billing_information:BillingInformation {price:"myPriceAbc".to_string(), unit:"unit".to_string(), messages:(&["message1".to_string()]).to_vec()},
            next_agent:AgentIdentification{name:"agent".to_string(), partner_id:"partner".to_string()}
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
