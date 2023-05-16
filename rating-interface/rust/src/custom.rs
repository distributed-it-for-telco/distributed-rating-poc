use crate::Bucket;
use wasmbus_rpc::actor::prelude::{RpcError, RpcResult};

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
