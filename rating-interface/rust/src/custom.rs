use crate::{Bucket, UsageProofRequest};
use serde_json::json;
use wasmbus_rpc::{
    actor::prelude::{RpcError, RpcResult},
    common::Context,
};
use wasmcloud_interface_keyvalue::{KeyValue, KeyValueSender, SetRequest};

impl Bucket {
    /// Create a new bucket by deserializing a JSON string
    pub fn try_from_str(bucket_json_str: &str) -> RpcResult<Bucket> {
        serde_json::from_str(bucket_json_str).map_err(|e| RpcError::Deser(e.to_string()))
    }

    /// Serialize the bucket to a JSON string
    pub fn serialize(&self) -> RpcResult<String> {
        serde_json::to_string(self).map_err(|e| RpcError::Ser(e.to_string()))
    }

    /// Get the bucket characteristic count
    pub fn characteristic_count(&self) -> u16 {
        self.bucket_characteristic.count
    }

    /// Clear the bucket characteristic count
    pub fn clear_characteristic_count(&mut self) {
        self.bucket_characteristic.count = 0;
    }

    /// Increment the bucket characteristic count
    pub fn increment_characteristic_count(&mut self) {
        self.bucket_characteristic.count += 1;
    }

    /// Decrement the bucket characteristic count
    pub fn decrement_characteristic_count(&mut self) {
        self.bucket_characteristic.count -= 1;
    }

    /// Set the bucket characteristic count
    pub fn set_characteristic_count(&mut self, count: u16) {
        self.bucket_characteristic.count = count;
    }
}

pub struct UsageProofHandler {}

impl UsageProofHandler {
    pub fn generate_rating_proof(usage_proof_request: &UsageProofRequest) -> String {
        let rating_date = "04/04/2023";

        let usage_template_str = json!({
            "id": usage_proof_request.usage_id,
            "usageDate": usage_proof_request.usage_date,
            "description": "Video on Demand with Bucket",
            "usageType": "VoD",
            "ratedProductUsage": {
                "isBilled": false,
                "ratingAmountType": "Total",
                "ratingDate": rating_date,
                "bucketValueConvertedInAmount": {
                    "unit": "EUR",
                    "value": usage_proof_request.rating
                },
                "productRef": {
                    "id": "1234",
                    "name": "Video on Demand with Bucket"
                }
            },
            "relatedParty": {
                "id": usage_proof_request.party_id
            },
            "usageCharacteristic": [
                {
                    "id": "122",
                    "name": "movie-count",
                    "valueType": "integer",
                    "value": usage_proof_request.usage
                }
            ]
        });

        usage_template_str.to_string()
    }
}

pub struct BucketAccessManager {}

impl BucketAccessManager {
    pub async fn get(_ctx: &Context, bucket_key: &str) -> RpcResult<Bucket> {
        let kv = KeyValueSender::new();
        let bucket_json_str = kv.get(_ctx, bucket_key).await?.value;
        let bucket: Bucket = Bucket::try_from_str(&bucket_json_str)?;

        Ok(bucket)
    }

    pub async fn save(_ctx: &Context, bucket_key: &str, bucket: &Bucket) -> RpcResult<()> {
        let serialized_bucket = bucket.serialize()?;

        let kv = KeyValueSender::new();
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
}
