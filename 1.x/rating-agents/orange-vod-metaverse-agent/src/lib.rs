
use crate::orange::commons::types::*;
use exports::orange::rating::ratingagent::Guest;
use wasi::logging::logging::{log,Level::Info};

use futures::executor::block_on;
use crate::orange::balancemanager::*;
use builders::RatingResponseBuilder;

mod builders;

wit_bindgen::generate!({
    generate_all,
    additional_derives: [serde::Serialize, serde::Deserialize]
});


struct OrangeVodMetaverseRatingagent;

// const BUCKET_KEY_PREFIX: &str = "bucket";
const ROOM_ENTRING_COST: f32 = 1.0;
const MOVIE_COST: f32 = 2.0;
const METAVERSE_OFFER_ID: &str = "metaverse-vod";
// const ROOM_OFFER_ID: &str = "metaverse-room-access";
const ROOM_USAGE_NAME: &str = "room-entering-usage";
const MOVIE_USAGE_NAME: &str = "movie-usage";


impl OrangeVodMetaverseRatingagent{
   async fn rate_usage_async(_request: RatingRequest) -> Option<RatingResponse> {
        log(Info, "", "Hello I'm your orange vod Metaverse agent");
        if let Some(first_usage) = _request.usage.usage_characteristic_list.first() {
            let advertiser_id = _request.customer_id;
            let offer_id = _request.offer_id;
            match first_usage.name.as_str() {
                ROOM_USAGE_NAME => {
                    return Some(Self::handle_room_rating( &advertiser_id, &offer_id, first_usage).await);
                }
                MOVIE_USAGE_NAME => {
                    return Some(Self::handle_movie_rating( &advertiser_id, &offer_id, first_usage).await);
                }
                _ => {
                    let usage = first_usage.value.parse::<f32>().unwrap();
                    log(Info, "", format!("Unknown usage: {}", usage).as_str());
                    return None;
                }
            }
        }
        return None;
    }
    
async fn handle_room_rating(
    advertiser_id: &String,
    offer_id: &String,
    usage_characteristic: &UsageCharacteristic,
) -> RatingResponse {
    Self::handle_rating(
        &advertiser_id,
        &offer_id,
        &usage_characteristic,
        ROOM_ENTRING_COST,
    ).await
}

async fn handle_movie_rating(
    advertiser_id: &String,
    offer_id: &String,
    usage_characteristic: &UsageCharacteristic,
) -> RatingResponse {
    Self::handle_rating(&advertiser_id, &offer_id, &usage_characteristic, MOVIE_COST).await
}

async fn handle_rating(
    customer_id: &String,
    _offer_id: &String,
    usage_characteristic: &UsageCharacteristic,
    unit_charge: f32,
) -> RatingResponse {
    let usage = usage_characteristic.value.parse::<f32>().unwrap();

    let mut rating_response_builder = RatingResponseBuilder::new();
    let balance = balancemanager::get_balance(customer_id, METAVERSE_OFFER_ID);
    

    let rating = Self::calc_rate(usage, unit_charge);

    if Self::has_sufficient_balance(balance.count, rating) {
        log(Info, "", format!( "Usage {} , Rating {} ", usage, rating).as_str());

        let new_balance: Balance = balancemanager::purchase(&balance, rating,customer_id, METAVERSE_OFFER_ID).expect("Bad balance");

        rating_response_builder
            .unit(new_balance.unit.clone())
            .price(rating.to_string())
            .message(&format!(
                "{} {} deducted from your balance",
                rating.to_string(),
                new_balance.unit
            ))
            .message(&format!(
                " Your Balance now is {} {}",
                new_balance.count,
                new_balance.unit
            ))
            .authorized();

            log(Info, "", 
            format!("Usage {} , Rating {} , & balance {}",
            usage, rating, new_balance.count).as_str());

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

    fn calc_rate(usage: f32, unit_charge: f32) -> f32 {
        unit_charge * usage
    }
    
    
}

impl Guest for OrangeVodMetaverseRatingagent {
    /// Say hello!
    fn rate_usage(_request: RatingRequest) -> RatingResponse {
        block_on(Self::rate_usage_async(_request)).unwrap()
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
export!(OrangeVodMetaverseRatingagent);
