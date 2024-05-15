use crate::{Bucket, KeyValueStoreWrapper};
use wasmbus_rpc::{
    actor::prelude::{RpcError, RpcResult},
    common::Context,
};

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

    pub fn characteristic_unit(&self) -> &str {
        self.bucket_characteristic.unit.as_str()
    }
    
}

pub struct BucketAccessManager {}

impl BucketAccessManager {
    pub async fn get(_ctx: &Context, bucket_key: &str) -> RpcResult<Bucket> {
        let bucket_json_str = KeyValueStoreWrapper::get(_ctx, bucket_key).await?;
        let bucket: Bucket = Bucket::try_from_str(&bucket_json_str)?;
        Ok(bucket)
    }

    pub async fn save(_ctx: &Context, bucket_key: &str, bucket: &Bucket) -> RpcResult<()> {
        let serialized_bucket = bucket.serialize()?;
        KeyValueStoreWrapper::put(_ctx, bucket_key, &serialized_bucket).await?;
        Ok(())
    }
}
