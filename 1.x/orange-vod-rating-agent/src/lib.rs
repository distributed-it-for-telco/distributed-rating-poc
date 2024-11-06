
use std::fmt;
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use wasi::logging::logging::{log,Level::Info};
use crate::orange::rating::types::*;
use exports::orange::rating::ratingagent::*;

wit_bindgen::generate!({
    generate_all,
    additional_derives: [serde::Serialize, serde::Deserialize]
});

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
#[derive(Serialize, Deserialize)]
struct Balance {
    count: f32,
    unit: String,
    party_id: String,
}

impl fmt::Display for Balance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} (Party ID: {})", self.count, self.unit, self.party_id)
    }
}

impl Balance {
    // Check if the balance is sufficient
    fn has_sufficient_balance(&self, amount: f32) -> bool {
        self.count >= amount
    }

    // Decrement the balance
    fn purchase(&mut self, amount: f32) -> Result<(), String> {
        if self.has_sufficient_balance(amount) {
            self.count -= amount;
            Ok(())
        } else {
            Err(String::from("Insufficient balance to purchase"))
        }
    }
}

struct MetaverseRatingagent;

impl Guest for MetaverseRatingagent {
    /// Say hello!
    fn rate_usage(_request: RatingRequest) -> RatingResponse {
        log(Info, "", &_request.offer_id);

        let bucket = wasi::keyvalue::store::open("").expect("failed to open empty bucket");
        let object_name = format!("{}:{}:{}", "balance", _request.customer_id, _request.offer_id);
        
        log(Info, "", &object_name);

        let balance_utf8 = bucket.get(&object_name).expect("couldn't retrieve count");
        let balance_str = String::from_utf8(balance_utf8.clone().unwrap()).unwrap();
        
        log(Info, "", &balance_str);

        let mut balance: Balance = serde_json::from_str(&balance_str).unwrap();
        log(Info, "", &balance.to_string());

        let price: f32 = 5.0;
        let purchase_amount = _request.usage.usage_characteristic_list[0].value.parse::<f32>().unwrap() * price;

        // Attempt to purchase
        let message: String;

        match balance.purchase(purchase_amount) {
            Ok(()) => {
                message = "Purchase successful".to_string();
                bucket.set(&object_name, &serde_json::to_vec(&balance).unwrap());
            },
            Err(err) => {
                message = "Purchase failed: ".to_string() + &err;
            },
        }

        RatingResponse {
            authorization_status: AuthorizationStatus {
                code: 12345,
                key: "two".to_string(),
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
