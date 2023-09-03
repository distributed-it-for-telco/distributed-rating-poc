use rating_interface::{
    Bucket, BucketAccessManager, RatingAgent, RatingAgentReceiver, RatingRequest, RatingResponse,
    RatingResponseBuilder, Usage, UsageCollector, UsageCollectorSender, UsageProofHandler,
    UsageProofRequest, ValidationRequest, ValidationResponse,
};

use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::info;
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
        let mut rating_response_builder = RatingResponseBuilder::new();

        /*
         *  Contract or Offer is bucket with 3 Movies equal 2 EURO
         */

        let bucket_key = format!(
            "{}:{}:{}",
            BUCKET_KEY_PREFIX,
            &_arg.customer_id.as_str(),
            OFFER_ID
        );
        let bucket = get_party_bucket(_ctx, bucket_key.as_str()).await?;
        let price: String;
        rating_response_builder
            .unit((&"EUR").to_string())
            .message(&"Your bucket is is 3 movies with price 2 EUR");

        if bucket.characteristic_count() == 0 {
            let rating = 2;
            info!("Handling rating empty buket");
            handle_rating(
                _ctx,
                &rating.to_string(),
                &_arg.customer_id,
                _arg.usage.clone(),
            )
            .await?;

            refill_bucket(_ctx, &bucket_key).await?;
            decrement_bucket(_ctx, &bucket_key).await?;

            price = rating.to_string();
        } else {
            let rating = 0;
            handle_rating(
                _ctx,
                &rating.to_string(),
                &_arg.customer_id,
                _arg.usage.clone(),
            )
            .await?;

            decrement_bucket(_ctx, &bucket_key).await?;

            price = rating.to_string();
            rating_response_builder.message(&format!(
                "Current price is {}, because it's part of package",
                rating
            ));
        }

        let rating_response = rating_response_builder
            .price(price.to_string())
            .authorized()
            .build();
        /*
         * Empty Response till we decide rating response how it should be
         */
        RpcResult::Ok(rating_response)
    }

    async fn validate(
        &self,
        ctx: &Context,
        arg: &ValidationRequest,
    ) -> RpcResult<ValidationResponse> {
        let mut validation_response: ValidationResponse = ValidationResponse::default();
        validation_response.next_agent = None;

        validation_response.valid = true;

        Ok(validation_response)
    }
}

async fn get_party_bucket(_ctx: &Context, bucket_key: &str) -> RpcResult<Bucket> {
    info!("Retreiving party bucket with key: {:?}", bucket_key);

    let bucket: Bucket = BucketAccessManager::get(_ctx, bucket_key).await?;

    info!("retrieved party bucket: {:?}", bucket);

    Ok(bucket)
}

async fn handle_rating(
    _ctx: &Context,
    _rating: &str,
    _party_id: &str,
    _usage: Usage,
) -> RpcResult<()> {
    let usage_date = "23/05/2023";
    let usage_id: String = generate_guid().await?;

    let usage_template_str = UsageProofHandler::generate_rating_proof(&UsageProofRequest {
        party_id: _party_id.to_owned(),
        rating: _rating.to_owned(),
        usage_characteristic_list: _usage.usage_characteristic_list.to_owned(),
        usage_id: usage_id.as_str().to_owned(),
        usage_date: usage_date.to_owned(),
        offer_id: OFFER_ID.to_owned(),
    });

    info!(
        "Sending usage proof to usage collector for party with id: {}",
        _party_id.to_string()
    );

    UsageCollectorSender::to_actor(&format!("mock/{}", "usage_collector_orange"))
        .store(_ctx, &usage_template_str)
        .await?;

    Ok(())
}

async fn refill_bucket(_ctx: &Context, bucket_key: &str) -> RpcResult<()> {
    info!("Refill bucket");

    let mut bucket: Bucket = BucketAccessManager::get(_ctx, bucket_key).await?;
    bucket.set_characteristic_count(3);

    BucketAccessManager::save(_ctx, bucket_key, &bucket).await?;

    Ok(())
}

async fn decrement_bucket(_ctx: &Context, bucket_key: &str) -> RpcResult<()> {
    let mut bucket: Bucket = BucketAccessManager::get(_ctx, bucket_key).await?;
    bucket.decrement_characteristic_count();

    BucketAccessManager::save(_ctx, bucket_key, &bucket).await?;

    Ok(())
}
