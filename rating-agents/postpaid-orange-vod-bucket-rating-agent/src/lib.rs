use rating_interface::{
    AuthorizationStatus, BillingInformation, RatingAgent, RatingAgentReceiver, RatingRequest,
    RatingResponse, UsageCollector, UsageCollectorSender, Bucket
};
use serde_json::{json};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_keyvalue::{KeyValueSender, KeyValue, SetRequest};
use wasmcloud_interface_logging::{info};
use wasmcloud_interface_numbergen::generate_guid;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, RatingAgent)]
struct PostpaidOrangeVodBucketRatingAgentActor {}

const BUCKET_KEY_PREFIX: &str = "bucket";

const OFFER_ID: &str = "2";

/// Implementation of Rating Agent trait methods
#[async_trait]
impl RatingAgent for PostpaidOrangeVodBucketRatingAgentActor {
    async fn rate_usage(&self, _ctx: &Context, _arg: &RatingRequest) -> RpcResult<RatingResponse> {
        info!("Hello I'm your orange postpaid vod bucket rating agent");

        /*
        *  Contract or Offer is bucket with 3 Movies equal 2 EURO
        */

        let bucket_key = format!("{}:{}:{}", BUCKET_KEY_PREFIX, &_arg.customer_id.as_str(), OFFER_ID);

        let bucket = get_party_bucket(_ctx, bucket_key.as_str()).await?;

        let mut billing_info = BillingInformation::default();
        billing_info.unit = (&"EUR").to_string();
        billing_info.messages = vec![String::from("Your bucket is is 3 movies with price 2 EUR")];

        if bucket.bucket_characteristic.count == 0 {
            let rating = 2;
            info!("Handling rating empty buket");
            handle_rating(_ctx, &rating.to_string(), &_arg.customer_id, &_arg.usage).await?;

            refill_bucket(_ctx, &bucket_key).await?;
            decrement_bucket(_ctx, &bucket_key).await?;

            billing_info.price = rating.to_string();
        } else {
            let rating = 0;
            handle_rating(_ctx, &rating.to_string(), &_arg.customer_id, &_arg.usage).await?;

            decrement_bucket(_ctx, &bucket_key).await?;

            billing_info.price = rating.to_string();
            billing_info.messages.push(
                String::from(format!("Current price is {}, because it's part of package", rating))
            );
        }

        /*
         * Empty Response till we decide rating response how it should be
         */
        RpcResult::Ok(RatingResponse {
            authorization_status: AuthorizationStatus::default(),
            billing_information: billing_info
        })
    }
}

async fn get_party_bucket(
    _ctx: &Context,
    bucket_key: &str
) -> RpcResult<Bucket> {
    let kv = KeyValueSender::new();

    info!("Retreiving party bucket with key: {:?}", bucket_key);

    let bucket_json_str = kv.get(_ctx, bucket_key).await?.value;

    let bucket: Bucket = serde_json::from_str(&bucket_json_str)
        .map_err(|err| RpcError::Ser(format!("{}", err)))?;

    info!("retrieved party bucket: {:?}", bucket);

    Ok(bucket)
}

async fn handle_rating(
    _ctx: &Context,
    _rating: &str,
    _party_id: &str,
    _usage: &str
) -> RpcResult<()> {
    let usage_date = "04/04/2023";
    let usage_id: String = generate_guid().await
        .map_err(|err| RpcError::Ser(format!("{}", err)))?;

    let rating_date = "04/04/2023";

    let usage_template_str = json!({
        "id": usage_id,
        "usageDate": usage_date,
        "description": "Video on Demand with Bucket",
        "usageType": "VoD",
        "ratedProductUsage": {
            "isBilled": false,
            "ratingAmountType": "Total",
            "ratingDate": rating_date,
            "bucketValueConvertedInAmount": {
                "unit": "EUR",
                "value": _rating
            },
            "productRef": {
                "id": "1234",
                "name": "Video on Demand with Bucket"
            }
        },
        "relatedParty": {
            "id": _party_id
        },
        "usageCharacteristic": [
            {
                "id": "122",
                "name": "movie-count",
                "valueType": "integer",
                "value": _usage
            }
        ]
    });

    info!(
        "Sending usage proof to usage collector for party with id: {}",
        _party_id.to_string()
    );

    UsageCollectorSender::to_actor(&format!("mock/{}", "usage_collector"))
        .store(_ctx, &usage_template_str)
        .await?;

    Ok(())

}

async fn refill_bucket(
    _ctx: &Context,
    bucket_key: &str
) -> RpcResult<()> {
    info!("Refill bucket");

    let kv = KeyValueSender::new();

    let bucket_json_str = kv.get(_ctx, bucket_key).await?.value;

    let mut bucket: Bucket = serde_json::from_str(&bucket_json_str)
        .map_err(|err| RpcError::Ser(format!("{}", err)))?;

    bucket.bucket_characteristic.count = 3;

    let serialized_bucket = serde_json::to_string(&bucket).map_err(|err| RpcError::Ser(format!("{}", err)))?;
    
    info!("serialized bucket after refill {:?}", serialized_bucket);

    kv.set(
        _ctx,
        &SetRequest {
            key: bucket_key.to_string(),
            value: serialized_bucket,
            expires: 0,
        },
    )
    .await?;

    Ok(())
}

async fn decrement_bucket(
    _ctx: &Context,
    bucket_key: &str
) -> RpcResult<()> {
    let kv = KeyValueSender::new();

    let bucket_json_str = kv.get(_ctx, bucket_key).await?.value;

    let mut bucket: Bucket = serde_json::from_str(&bucket_json_str).map_err(|err| RpcError::Ser(format!("{}", err)))?;

    bucket.bucket_characteristic.count -= 1;

    let serialized_bucket = serde_json::to_string(&bucket).map_err(|err| RpcError::Ser(format!("{}", err)))?;

    info!("serialized bucket after decrement {:?}", serialized_bucket);

    kv.set(
        _ctx,
        &SetRequest {
            key: bucket_key.to_string(),
            value: serialized_bucket,
            expires: 0,
        },
    )
    .await?;

    Ok(())
}