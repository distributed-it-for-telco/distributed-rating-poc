
use crate::orange::commons::types::*;
use crate::orange::commons::error_types::*;
use exports::orange::rating::ratingagent::Guest;
use wasi::logging::logging::{log,Level::Info};
use crate::orange::commons::rating_response_builder as RatingResponseBuilder;

use futures::executor::block_on;
use crate::orange::balancemanager::*;

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
   async fn rate_usage_async(_request: RatingRequest) -> Result<RatingResponse, UsageError>  {
        log(Info, "", "Hello I'm your orange vod Metaverse agent");
        if let Some(first_usage) = _request.usage.usage_characteristic_list.first() {
            let advertiser_id = _request.customer_id;
            let offer_id = _request.offer_id;
            match first_usage.name.as_str() {
                ROOM_USAGE_NAME => {
                    return Self::handle_room_rating( &advertiser_id, &offer_id, first_usage).await;
                }
                MOVIE_USAGE_NAME => {
                    return Self::handle_movie_rating( &advertiser_id, &offer_id, first_usage).await;
                }
                _ => {
                    let usage = first_usage.value.parse::<f32>().unwrap();
                    log(Info, "", format!("Unknown usage: {}", usage).as_str());
                }
            }
        }
        return Err(UsageError{message: "Unknown usage".to_string()});
    }
    
async fn handle_room_rating(
    advertiser_id: &String,
    offer_id: &String,
    usage_characteristic: &UsageCharacteristic,
) -> Result<RatingResponse, UsageError> {
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
) -> Result<RatingResponse, UsageError> {
    Self::handle_rating(&advertiser_id, &offer_id, &usage_characteristic, MOVIE_COST).await
}

async fn handle_rating(
    customer_id: &String,
    _offer_id: &String,
    usage_characteristic: &UsageCharacteristic,
    unit_charge: f32,
) ->  Result<RatingResponse, UsageError>  {
    let usage = usage_characteristic.value.parse::<f32>().unwrap();

    let balance = balancemanager::get_balance(customer_id, METAVERSE_OFFER_ID);
    

    let rating = Self::calc_rate(usage, unit_charge);

    let mut response_builder_handle = RatingResponseBuilder::create_builder();
    if Self::has_sufficient_balance(balance.count, rating) {
        log(Info, "", format!( "Usage {} , Rating {} ", usage, rating).as_str());

       
        let new_balance: Balance = balancemanager::purchase(&balance, rating,customer_id, METAVERSE_OFFER_ID).expect("Bad balance");

        response_builder_handle= RatingResponseBuilder::unit(response_builder_handle, &new_balance.unit);
        response_builder_handle= RatingResponseBuilder::price(response_builder_handle, &rating.to_string());
        response_builder_handle= RatingResponseBuilder::message(response_builder_handle, &format!(
            "{} {} deducted from your balance",
            rating.to_string(),
            new_balance.unit
        ));
        response_builder_handle= RatingResponseBuilder::message(response_builder_handle, &format!(
            " Your Balance now is {} {}",
            new_balance.count,
            new_balance.unit
        ));
        response_builder_handle= RatingResponseBuilder::authorized(response_builder_handle);
        
        log(Info, "", 
        format!("Usage {} , Rating {} , & balance {}",
        usage, rating, new_balance.count).as_str());

    } else {
        response_builder_handle= RatingResponseBuilder::message(response_builder_handle,&"You have no sufficient balance");
        response_builder_handle= RatingResponseBuilder::not_authorized(response_builder_handle);
    }
    Ok(RatingResponseBuilder::build(response_builder_handle))
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
    fn rate_usage(_request: RatingRequest) -> Result<RatingResponse, UsageError>  {
        block_on(Self::rate_usage_async(_request))
    }
    

    fn validate(_request: ValidationRequest) -> Result<ValidationResponse, ValidationError>   {
        Ok(ValidationResponse { valid: true })
    }

    fn get_children(_request: GetChildrenRequest) -> AgentList {
        AgentList {
            agents: vec![],
        }
    }

}
export!(OrangeVodMetaverseRatingagent);
