wit_bindgen::generate!({
    generate_all,
    additional_derives: [serde::Serialize, serde::Deserialize]
});

use std::collections::HashMap;
use lazy_static::lazy_static;
use futures::executor::block_on;
use wasi::logging::logging::{log,Level::Info};
use crate::orange::commons::commons::{generate_rating_proof,UsageProofRequest};
use crate::orange::rating::types::*;
use crate::orange::rating::types::{AgentList,RatingRequest, RatingResponse, GetChildrenRequest,ValidationRequest, ValidationResponse};
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
    async fn rate_usage_async(request: RatingRequest) -> RatingResponse {
        log(Info,"","Hello I'm your orange postpaid vod rating agent");
        let usage_date = "04/04/2023";
        let usage_id: String = "".to_string();// generate_guid().await?;
        
        /*
         *  Contract or Offer is 10% added to provider price
         */


        // let previouse_rating_price_str = request.rating_history.clone().pop().unwrap().price;
        let previouse_rating_price = 12.5;//previouse_rating_price_str.parse::<f64>().unwrap();
        let rating = previouse_rating_price + (previouse_rating_price * RATE_FEE);

        let _usage_template_str = generate_rating_proof(&UsageProofRequest {
            party_id: request.customer_id.to_owned(),
            rating: rating.to_string(),
            usage_characteristic_list: Self::map_usage_characteristic_list(request.usage.usage_characteristic_list.to_owned()),
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
        let rating_response = RatingResponse{
                authorization_status: AuthorizationStatus{
                    code: 200,
                    key: "".to_string()
                },
                billing_information: BillingInformation{
                    messages: vec![
                        "You can now enjoy your movie on Streamzie".to_string(),
                        format!("The cost of this transaction is {} EUR",rating.to_string())
                    ],
                    price: rating.to_string(),
                    unit: "EUR".to_string()
                },
                next_agent: AgentIdentification{
                    name: "".to_string(),
                    partner_id: "".to_string()
                }
            };
        rating_response
    }

    fn map_usage_characteristic_list(usage_characteristic_list: Vec<orange::rating::types::UsageCharacteristic>) -> Vec<orange::commons::types::UsageCharacteristic>{
        fn map_item(item: orange::rating::types::UsageCharacteristic) -> orange::commons::types::UsageCharacteristic {
            orange::commons::types::UsageCharacteristic{
                name: item.name,
                value: item.value,
                value_type: item.value_type
            }
        }
        usage_characteristic_list.iter().map(|item: &orange::rating::types::UsageCharacteristic| map_item(item.clone())).collect::<Vec<orange::commons::types::UsageCharacteristic>>()
    }


    async fn validate_async(request: ValidationRequest) -> ValidationResponse {
        let mut validation_response: ValidationResponse = ValidationResponse{
            valid: true
        };
        if !request.client_country.is_empty() && request.client_country.to_owned() == "EG" {
            validation_response.valid = true;
        } else {
            validation_response.valid = false;
        }
        validation_response
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
    fn rate_usage(request: RatingRequest) -> RatingResponse {
        block_on(Self::rate_usage_async(request))
    }
    fn validate(request: ValidationRequest) -> ValidationResponse {
        block_on(Self::validate_async(request))
    }
    fn get_children(request: GetChildrenRequest) -> AgentList {
        block_on(Self::get_children_async(request))
    }

    // fn rate_usage(_request: RatingRequest) -> RatingResponse {
    //     log(Info, "", &_request.offer_id);

    //     let bucket = wasi::keyvalue::store::open("").expect("failed to open empty bucket");
    //     let object_name = format!("{}:{}:{}", "balance", _request.customer_id, _request.offer_id);
        
    //     log(Info, "", &object_name);

    //     let balance_utf8 = bucket.get(&object_name).expect("couldn't retrieve count");
    //     let balance_str = String::from_utf8(balance_utf8.clone().unwrap()).unwrap();
        
    //     log(Info, "", &balance_str);

    //     let mut balance: Balance = serde_json::from_str(&balance_str).unwrap();
    //     log(Info, "", &balance.to_string());

    //     let price: f32 = 5.0;
    //     let purchase_amount = _request.usage.usage_characteristic_list[0].value.parse::<f32>().unwrap() * price;

    //     // Attempt to purchase
    //     let message: String;

    //     match balance.purchase(purchase_amount) {
    //         Ok(()) => {
    //             message = "Purchase successful".to_string();
    //             bucket.set(&object_name, &serde_json::to_vec(&balance).unwrap());
    //         },
    //         Err(err) => {
    //             message = "Purchase failed: ".to_string() + &err;
    //         },
    //     }

    //     RatingResponse {
    //         authorization_status: AuthorizationStatus {
    //             code: 12345,
    //             key: "two".to_string(),
    //         },
    //         billing_information: BillingInformation {
    //             price: price.to_string(),
    //             unit: balance.unit.to_string(),
    //             messages: vec![message],
    //         },
    //         next_agent: AgentIdentification {
    //             name: "agent".to_string(),
    //             partner_id: "partner".to_string(),
    //         },
    //     }
    // }

    // fn validate(_request: ValidationRequest) -> ValidationResponse {
    //     ValidationResponse { valid: true }
    // }

    // fn get_children(_request: GetChildrenRequest) -> AgentList {
    //     AgentList {
    //         agents: vec![],
    //     }
    // }
}

export!(OrangeVodRatingagent);
