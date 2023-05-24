use rating_interface::{
    AuthorizationStatus, BillingInformation, Bucket, RatingAgent, RatingAgentReceiver,
    RatingRequest, RatingResponse, UsageCollector, UsageCollectorSender, UsageProofHandler,
    UsageProofRequest,
};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_keyvalue::{KeyValue, KeyValueSender, SetRequest};
use wasmcloud_interface_logging::info;
use wasmcloud_interface_numbergen::generate_guid;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, RatingAgent)]
struct PostpaidOrangeVodThresholdDiscountAgentActor {}

const BUCKET_KEY_PREFIX: &str = "bucket";

const OFFER_ID: &str = "3";

const THRESHOLD: u16 = 10;

const MOVIE_COST: f64 = 2.0;

const THRESHOLD_DISCOUNT: f64 = 0.10;

/// Implementation of RatingAgent trait methods
#[async_trait]
impl RatingAgent for PostpaidOrangeVodThresholdDiscountAgentActor {
    async fn rate_usage(&self, _ctx: &Context, _arg: &RatingRequest) -> RpcResult<RatingResponse> {
        info!("Hello I'm your orange postpaid vod Threshold discount rating agent");

        /*
         *  Contract or Offer is normal movie cost = 2 EU , if you watched 10 movies the 11th will be with discount 10%
         */

        let bucket_key = format!(
            "{}:{}:{}",
            BUCKET_KEY_PREFIX,
            &_arg.customer_id.as_str(),
            OFFER_ID
        );
        let bucket = get_party_bucket(_ctx, bucket_key.as_str()).await?;
        let free_text: String;
        let mut billing_info = BillingInformation::default();
        billing_info.unit = (&"EUR").to_string();

        if bucket.characteristic_count() == THRESHOLD {
            let rating = MOVIE_COST - MOVIE_COST * THRESHOLD_DISCOUNT;
            handle_rating(_ctx, &rating.to_string(), &_arg.customer_id, &_arg.usage).await?;

            clear_bucket(_ctx, &bucket_key).await?;
            billing_info.price = rating.to_string();
            free_text = format!("You got discount {}% ", THRESHOLD_DISCOUNT * 100.0);
        } else {
            let rating = MOVIE_COST;
            handle_rating(_ctx, &rating.to_string(), &_arg.customer_id, &_arg.usage).await?;
            increase_bucket(_ctx, &bucket_key).await?;
            billing_info.price = rating.to_string();
            if bucket.characteristic_count() + 1 == THRESHOLD {
                free_text = format!(
                    "You will get discount {}% Next movie",
                    THRESHOLD_DISCOUNT * 100.0
                );
            } else {
                free_text = format!(
                    "Still you can get discount {}% after watching {} movies",
                    THRESHOLD_DISCOUNT * 100.0,
                    (THRESHOLD - (bucket.bucket_characteristic.count + 1))
                )
            }
        }

        billing_info.messages.push(free_text.to_string());
        RpcResult::Ok(RatingResponse {
            authorization_status: AuthorizationStatus::default(),
            billing_information: billing_info,
        })
    }
}

async fn get_party_bucket(_ctx: &Context, bucket_key: &str) -> RpcResult<Bucket> {
    info!("Start get_party_bucket");
    let bucket: Bucket = BucketAccessManager::get(_ctx, bucket_key).await?;
    info!("End get_party_bucket");
    Ok(bucket)
}

async fn increase_bucket(_ctx: &Context, bucket_key: &str) -> RpcResult<()> {
    let mut bucket: Bucket = BucketAccessManager::get(_ctx, bucket_key).await?;

    bucket.increment_characteristic_count();

    BucketAccessManager::save(_ctx, bucket_key, &bucket).await?;

    Ok(())
}

async fn clear_bucket(_ctx: &Context, bucket_key: &str) -> RpcResult<()> {
    let mut bucket: Bucket = BucketAccessManager::get(_ctx, bucket_key).await?;
    
    bucket.clear_characteristic_count();
    BucketAccessManager::save(_ctx, bucket_key, &bucket).await?;

    Ok(())
}

async fn handle_rating(
    _ctx: &Context,
    _rating: &str,
    _party_id: &str,
    _usage: &str,
) -> RpcResult<()> {
    let usage_date = "22/05/2023";
    let usage_id: String = generate_guid().await?;

    let usage_template_str = UsageProofHandler::generate_rating_proof(&UsageProofRequest {
        party_id: _party_id.to_owned(),
        rating: _rating.to_owned(),
        usage: _usage.to_owned(),
        usage_id: usage_id.as_str().to_owned(),
        usage_date: usage_date.to_owned(),
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
