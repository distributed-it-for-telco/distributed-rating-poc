
use crate::orange::rating::types::*;
use exports::orange::rating::ratingagent::*;
use wasi::logging::logging::log;
use std::fmt;
use serde::{Serialize, Deserialize};
use crate::orange::balancemanager::*;

wit_bindgen::generate!({
    generate_all,
    additional_derives: [serde::Serialize, serde::Deserialize]
});


struct OrangeVodMetaverseRatingagent;

// const BUCKET_KEY_PREFIX: &str = "bucket";
const ROOM_ENTRING_COST: f32 = 1.0;
const MOVIE_COST: f32 = 2.0;
const MOVIE_OFFER_ID: &str = "metaverse-vod";
// const ROOM_OFFER_ID: &str = "metaverse-room-access";
const ROOM_USAGE_NAME: &str = "room-entering-usage";
const MOVIE_USAGE_NAME: &str = "movie-usage";


impl OrangeVodMetaverseRatingagent{
    
    
async fn handle_room_rating(
    advertiser_id: &String,
    usage_characteristic: &UsageCharacteristic,
) -> RatingResponse {
    Self::handle_rating(
        &advertiser_id,
        &usage_characteristic,
        ROOM_ENTRING_COST,
    ).await
}

async fn handle_movie_rating(
    customer_id: &String,
    usage_characteristic: &UsageCharacteristic,
) -> RatingResponse {
    Self::handle_rating(&customer_id, &usage_characteristic, MOVIE_COST).await
}

async fn handle_rating(
    customer_id: &String,
    usage_characteristic: &UsageCharacteristic,
    unit_charge: f32,
) -> RatingResponse {
    let usage = usage_characteristic.value.parse::<f32>().unwrap();

    let mut rating_response_builder = RatingResponseBuilder::new();
    let balance_access_manager = BalanceAccessManager::default();

    let balance = balance_access_manager
        .get_balance( customer_id, MOVIE_OFFER_ID)
        .await;

    let rating = Self::calc_rate(usage, unit_charge);

    // not depending on the balance <=0  but calc rate and keep sufficient
    //balance in has_sufficient_balance to centralize the decision..
    // may be we have a case the customer has 0 balance but he still can use the service ...... [to be validated]

    if Self::has_sufficient_balance(balance.balance_characteristic.count, rating) {
        log(wasi::logging::logging::Level::Info, "", 
            format!( "Usage {} , Rating {} , & balance {}",
            usage, rating, balance.balance_characteristic.count).as_str());
        

        let new_balance: Balance = balance_access_manager
            .withdraw(customer_id, MOVIE_OFFER_ID, rating)
            .await;

        rating_response_builder
            .unit(new_balance.balance_unit().to_string())
            .price(rating.to_string())
            .message(&format!(
                "{} {} deducted from your balance",
                rating.to_string(),
                new_balance.balance_unit()
            ))
            .message(&format!(
                " Your Balance now is {} {}",
                new_balance.balance_count(),
                new_balance.balance_unit()
            ))
            .authorized();

            log(wasi::logging::logging::Level::Info, "", 
            format!("Usage {} , Rating {} , & balance {}",
            usage, rating, new_balance.balance_count()).as_str());

    } else {
        rating_response_builder
            .message(&"You have no sufficient balance")
            .not_authorized();
    }
    let rating_response = rating_response_builder.build();

    rating_response
    }

    fn has_sufficient_balance(balance: f32, charge: f32) -> bool {
        if balance < charge {
            return false;
        } else {
            return true;
        }
    }

    fn calc_rate(usage: f32, UNIT_CHARGE: f32) -> f32 {
        UNIT_CHARGE * usage
    }
    
    
}

impl Guest for OrangeVodMetaverseRatingagent {
    /// Say hello!
    fn rate_usage(_request: RatingRequest) -> RatingResponse {
        log(wasi::logging::logging::Level::Info, "", "Hello I'm your orange vod Metaverse agent");
        if let Some(first_usage) = _request.usage.usage_characteristic_list.first() {
            let advertiser_id = _request.customer_id;
            match first_usage.name.as_str() {
                ROOM_USAGE_NAME => {
                    return Self::handle_room_rating( &advertiser_id, first_usage).await;
                }
                MOVIE_USAGE_NAME => {
                    return Self::handle_movie_rating( &advertiser_id, first_usage).await;
                }
                _ => {
                    let usage = first_usage.value.parse::<f32>().unwrap();
                    log(wasi::logging::logging::Level::Info, "", format!("Unknown usage: {}", usage).as_str());
                }
            }
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

// fn rate_usage(_request: RatingRequest) -> RatingResponse {
    //     let balance = balancemanager::get_balance(&_request.customer_id, &_request.offer_id);
    //     log(wasi::logging::logging::Level::Info, "", &balance.count.to_string());

    //     let price: f32 = 5.0;
    //     let purchase_amount = _request.usage.usage_characteristic_list[0].value.parse::<f32>().unwrap() * price;

    //     // Attempt to purchase
    //     let message: String;

    //     match balancemanager::purchase(&balance, purchase_amount, &_request.customer_id, &_request.offer_id) {
    //         Ok(updated_balance) => {
    //             message = "Purchase successful".to_string();
    //         },
    //         Err(err) => {
    //             message = "Purchase failed: ".to_string() + &err;
    //         },
    //     }

    //     RatingResponse {
    //         authorization_status: AuthorizationStatus {
    //             code: 12345,
    //             key: "hello".to_string(),
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
}
export!(OrangeVodMetaverseRatingagent);
