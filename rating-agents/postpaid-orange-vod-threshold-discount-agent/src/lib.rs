use rating_interface::{
    AuthorizationStatus, BillingInformation, RatingAgent, RatingAgentReceiver, RatingRequest,
    RatingResponse, UsageCollector, UsageCollectorSender,
};
use serde::{Serialize, Deserialize};
use serde_json::{json};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_keyvalue::{KeyValueSender, KeyValue, SetRequest};
use wasmcloud_interface_logging::info;
use wasmcloud_interface_numbergen::generate_guid;


// Bucket definition
#[derive(Serialize, Deserialize, Debug)]
struct Bucket {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "offerId")]
    offer_id: String,
    #[serde(rename = "partyId")]
    party_id: String,
    #[serde(rename = "bucketCharacteristic")]
    bucket_characteristic: BucketCharacteristic,
}

#[derive(Serialize, Deserialize, Debug)]
struct BucketCharacteristic {
    #[serde(rename = "unit")]
    unit: String,
    #[serde(rename = "count")]
    pub count: u32,
}

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, RatingAgent)]
struct PostpaidOrangeVodThresholdDiscountAgentActor {}


const BUCKET_KEY_PREFIX: &str = "bucket";

const OFFER_ID: &str = "3";

const THRESHOLD:u32=10;

const MOVIE_COST:f64=2.0;

const THRESHOLD_DISCOUNT:f64=0.10;

/// Implementation of RatingAgent trait methods
#[async_trait]
impl RatingAgent for PostpaidOrangeVodThresholdDiscountAgentActor {
   
    async fn rate_usage(&self, _ctx: &Context, _arg: &RatingRequest) -> RpcResult<RatingResponse> {

        info!("Hello I'm your orange postpaid vod Threshold discount rating agent");

        /*
        *  Contract or Offer is normal movie cost = 2 EU , if you watched 10 movies the 11th will be with discount 10%
        */

        let bucket_key = format!("{}:{}:{}", BUCKET_KEY_PREFIX, &_arg.customer_id.as_str(), OFFER_ID);
        let bucket = get_party_bucket(_ctx, bucket_key.as_str()).await?;
        let mut free_text: String = "".to_string();
        let mut billing_info = BillingInformation::default();
        billing_info.unit = (&"EUR").to_string();
       
      
        if bucket.bucket_characteristic.count == THRESHOLD {
        
            let rating = MOVIE_COST - MOVIE_COST*THRESHOLD_DISCOUNT;
            handle_rating(_ctx, &rating.to_string(), &_arg.customer_id, &_arg.usage).await?;

            clear_bucket(_ctx, &bucket_key).await?;
            billing_info.price = rating.to_string();
            free_text = format!("You got discount {}% ",THRESHOLD_DISCOUNT*100.0);
            
            
            
        } else {
            let rating = MOVIE_COST;
            handle_rating(_ctx, &rating.to_string(), &_arg.customer_id, &_arg.usage).await?;
            increase_bucket(_ctx, &bucket_key).await?;
            billing_info.price = rating.to_string();
            if bucket.bucket_characteristic.count+1 == THRESHOLD {
                free_text=format!("You will get discount {}% Next movie",THRESHOLD_DISCOUNT*100.0);
            } else {
                free_text =format!("Still you can get discount {}% after watching {} movies",THRESHOLD_DISCOUNT*100.0,(THRESHOLD - (bucket.bucket_characteristic.count+1)))
            } 
         
        }

         
        billing_info.messages.push(
            free_text.to_string()
        );
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
    info!("Start get_party_bucket");
    let kv = KeyValueSender::new();
    let bucket_json_str = kv.get(_ctx, bucket_key).await?.value;
    info!("bucket {}", bucket_json_str);

    let bucket: Bucket = serde_json::from_str(&bucket_json_str).map_err(|err| RpcError::Ser(format!("{}", err)))?;
    info!("End get_party_bucket");
    Ok(bucket)
}


async fn increase_bucket(
    _ctx: &Context,
    bucket_key: &str
) -> RpcResult<()> {
    let kv = KeyValueSender::new();

    let bucket_json_str = kv.get(_ctx, bucket_key).await?.value;

    let mut bucket: Bucket = serde_json::from_str(&bucket_json_str).map_err(|err| RpcError::Ser(format!("{}", err)))?;

    bucket.bucket_characteristic.count += 1;

    let serialized_bucket = serde_json::to_string(&bucket).map_err(|err| RpcError::Ser(format!("{}", err)))?;

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


async fn clear_bucket(
    _ctx: &Context,
    bucket_key: &str
) -> RpcResult<()> {
    let kv = KeyValueSender::new();

    let bucket_json_str = kv.get(_ctx, bucket_key).await?.value;

    let mut bucket: Bucket = serde_json::from_str(&bucket_json_str).map_err(|err| RpcError::Ser(format!("{}", err)))?;

    bucket.bucket_characteristic.count = 0;


    let serialized_bucket = serde_json::to_string(&bucket).map_err(|err| RpcError::Ser(format!("{}", err)))?;

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
        "description": "Video on Demand",
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
                "name": "Video on Demand"
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
