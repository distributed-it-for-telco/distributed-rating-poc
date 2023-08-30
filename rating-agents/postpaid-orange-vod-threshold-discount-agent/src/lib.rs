use rating_interface::{
    Bucket, BucketAccessManager, RatingAgent, RatingAgentReceiver, RatingRequest, RatingResponse,
    RatingResponseBuilder, Usage, UsageCollector, UsageCollectorSender, UsageProofHandler,
    UsageProofRequest, ValidationRequest, ValidationResponse,GetChildrenRequest,AgentList
};
use wasmbus_rpc::actor::prelude::*;
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
const RATING_PROOF_DESC: &str = "Video on Demand with Threshold Pricing";
const RATING_PROOF_USAGE_TYPE: &str = "VoDThreshold";
const RATING_PROOF_PRODUCT_NAME: &str = "Video on Demand with Threshold Pricing";

/// Implementation of RatingAgent trait methods
#[async_trait]
impl RatingAgent for PostpaidOrangeVodThresholdDiscountAgentActor {
    async fn rate_usage(&self, _ctx: &Context, _arg: &RatingRequest) -> RpcResult<RatingResponse> {
        info!("Hello I'm your orange postpaid vod Threshold discount rating agent");

        let mut rating_response_builder = RatingResponseBuilder::new();

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
        let unit = (&"EUR").to_string();
        let price: String;

        if bucket.characteristic_count() == THRESHOLD {
            let rating = MOVIE_COST - MOVIE_COST * THRESHOLD_DISCOUNT;
            handle_rating(_ctx, &rating.to_string(), &_arg.customer_id, &_arg.usage).await?;
            clear_bucket(_ctx, &bucket_key).await?;
            price = rating.to_string();
            free_text = format!("You got discount {}% ", THRESHOLD_DISCOUNT * 100.0);
        } else {
            let rating = MOVIE_COST;
            handle_rating(_ctx, &rating.to_string(), &_arg.customer_id, &_arg.usage).await?;
            increase_bucket(_ctx, &bucket_key).await?;
            price = rating.to_string();
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

        let rating_response = rating_response_builder
            .unit(unit.to_string())
            .price(price.to_string())
            .message(&free_text.to_string())
            .authorized()
            .build();

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

    async fn get_children(&self, ctx: &Context, arg: &GetChildrenRequest) -> RpcResult<AgentList> {
        Ok(AgentList::new())
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
    _usage: &Usage,
) -> RpcResult<()> {
    let usage_date = "22/05/2023";
    let usage_id: String = generate_guid().await?;

    let usage_template_str = UsageProofHandler::generate_rating_proof(&UsageProofRequest {
        party_id: _party_id.to_owned(),
        rating: _rating.to_owned(),
        usage_characteristic_list: _usage.usage_characteristic_list.to_owned(),
        usage_id: usage_id.as_str().to_owned(),
        usage_date: usage_date.to_owned(),
        offer_id: OFFER_ID.to_owned(),
        description: RATING_PROOF_DESC.to_owned(),
        usage_type: RATING_PROOF_USAGE_TYPE.to_owned(),
        product_name: RATING_PROOF_PRODUCT_NAME.to_owned(),
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
