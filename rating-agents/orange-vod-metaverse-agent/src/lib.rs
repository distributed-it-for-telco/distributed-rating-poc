use rating_interface::{
    AgentList, Balance, BalanceAccessManager, Bucket, BucketAccessManager, GetChildrenRequest,
    RatingAgent, RatingAgentReceiver, RatingRequest, RatingResponse, RatingResponseBuilder,
    UsageCharacteristic, ValidationRequest, ValidationResponse,
};

use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::info;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, RatingAgent)]
struct OrangeVodMetaverseAgentActor {}

// const BUCKET_KEY_PREFIX: &str = "bucket";
const ROOM_ENTRING_COST: f32 = 1.0;
const MOVIE_COST: f32 = 2.0;
const MOVIE_OFFER_ID: &str = "metaverse-vod";
// const ROOM_OFFER_ID: &str = "metaverse-room-access";
const ROOM_USAGE_NAME: &str = "room-entering-usage";
const MOVIE_USAGE_NAME: &str = "movie-usage";

#[async_trait]
//adds a function to the struct
impl RatingAgent for OrangeVodMetaverseAgentActor {
    async fn rate_usage(&self, _ctx: &Context, _arg: &RatingRequest) -> RpcResult<RatingResponse> {
        info!("Hello I'm your orange vod Metaverse agent");

        if let Some(first_usage) = _arg.usage.usage_characteristic_list.first() {
            let advertiser_id = &_arg.customer_id;

            match first_usage.name.as_str() {
                ROOM_USAGE_NAME => {
                    return handle_room_rating(_ctx, advertiser_id, first_usage).await;
                }
                MOVIE_USAGE_NAME => {
                    return handle_movie_rating(_ctx, advertiser_id, first_usage).await;
                }
                _ => {
                    let usage = first_usage.value.parse::<f32>().unwrap();
                    info!("Unknown usage: {}", usage);
                }
            }
        }

        RpcResult::from(Err(RpcError::Other(format!(
            "Unknown usage name {}",
            _arg.usage.usage_characteristic_list.first().unwrap().name
        ))))

        // let usage: f32 = _arg.usage.parse().unwrap();
    }

    async fn validate(
        &self,
        _ctx: &Context,
        _arg: &ValidationRequest,
    ) -> RpcResult<ValidationResponse> {
        let mut validation_response: ValidationResponse = ValidationResponse::default();

        validation_response.valid = true;

        Ok(validation_response)
    }

    async fn get_children(&self, _ctx: &Context, _arg: &GetChildrenRequest) -> RpcResult<AgentList> {
        Ok(AgentList::new())
    }
}

async fn handle_room_rating(
    _ctx: &Context,
    advertiser_id: &String,
    usage_characteristic: &UsageCharacteristic,
) -> RpcResult<RatingResponse> {
   return handle_rating(&_ctx, &advertiser_id, &usage_characteristic,ROOM_ENTRING_COST).await;
}

// async fn handle_room_rating(
//     _ctx: &Context,
//     advertiser_id: &String,
//     usage_characteristic: &UsageCharacteristic,
// ) -> RpcResult<RatingResponse> {
//     let usage = usage_characteristic.value.parse::<f32>().unwrap();

//     let mut rating_response_builder = RatingResponseBuilder::new();

//     let bucket_key = format!("{}:{}:{}", BUCKET_KEY_PREFIX, advertiser_id, ROOM_OFFER_ID);

//     let mut bucket: Bucket = BucketAccessManager::get(_ctx, bucket_key.as_str()).await?;

//     // not depending on the balance <=0  but calc rate and keep sufficient
//     //balance in has_sufficient_balance to centralize the decision..
//     // may be we have a case the customer has 0 balance but he still can use the service ...... [to be validated]

//     if bucket.characteristic_count() > 0 {
//         info!(
//             "Usage {} , Bucket {} ",
//             usage,
//             bucket.characteristic_count()
//         );

//         bucket.decrement_characteristic_count();

//         BucketAccessManager::save(_ctx, bucket_key.as_str(), &bucket).await?;

//         rating_response_builder
//             .unit(bucket.characteristic_unit().to_string())
//             .price((&"1").to_string())
//             .message(&format!(
//                 " Your Bucket now has {} {}",
//                 bucket.characteristic_count(),
//                 bucket.characteristic_unit()
//             ))
//             .authorized();

//         info!(
//             "Usage {} , Bucket {} ",
//             usage,
//             bucket.characteristic_count()
//         );
//     } else {
//         rating_response_builder
//             .message(&"You have insufficient room access tokens in your bucket")
//             .not_authorized();
//     }
//     let rating_response = rating_response_builder.build();

//     RpcResult::Ok(rating_response)
// }

async fn handle_movie_rating(
    _ctx: &Context,
    customer_id: &String,
    usage_characteristic: &UsageCharacteristic,
) -> RpcResult<RatingResponse> {
    return handle_rating(&_ctx, &customer_id, &usage_characteristic,MOVIE_COST).await;
}

async fn handle_rating(
    _ctx: &Context,
    customer_id: &String,
    usage_characteristic: &UsageCharacteristic,
    decrement_cost: f32
) -> RpcResult<RatingResponse> {
    let usage = usage_characteristic.value.parse::<f32>().unwrap();

    let mut rating_response_builder = RatingResponseBuilder::new();
    let balance_access_manager = BalanceAccessManager::default();

    let balance = balance_access_manager
        .get_balance(_ctx, customer_id, MOVIE_OFFER_ID)
        .await?;

    let rating = calc_rate(usage,decrement_cost);

    // not depending on the balance <=0  but calc rate and keep sufficient
    //balance in has_sufficient_balance to centralize the decision..
    // may be we have a case the customer has 0 balance but he still can use the service ...... [to be validated]

    if has_sufficient_balance(balance.balance_characteristic.count, rating) {
        info!(
            "Usage {} , Rating {} , & balance {}",
            usage, rating, balance.balance_characteristic.count
        );

        let new_balance: Balance = balance_access_manager
            .withdraw(_ctx, customer_id, MOVIE_OFFER_ID, rating)
            .await?;

        rating_response_builder
            .unit(new_balance.balance_unit().to_string())
            .price(rating.to_string())
            .message(&format!(
                "{} {} deducted from your balance",
                rating.to_string(), new_balance.balance_unit()
            ))
            .message(&format!(
                " Your Balance now is {} {}",
                new_balance.balance_count(), new_balance.balance_unit()
            ))
            .authorized();

        info!(
            "Usage {} , Rating {} , & balance {}",
            usage, rating, new_balance.balance_count()
        );
    } else {
        rating_response_builder
            .message(&"You have no sufficient balance")
            .not_authorized();
    }
    let rating_response = rating_response_builder.build();

    RpcResult::Ok(rating_response)
}

fn has_sufficient_balance(balance: f32, charge: f32) -> bool {
    if balance < charge {
        return false;
    } else {
        return true;
    }
}

fn calc_rate(usage: f32,DECREMENT_COST: f32) -> f32 {
    DECREMENT_COST * usage
}
