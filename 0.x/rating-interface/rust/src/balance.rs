use crate::{Balance, KeyValueStoreWrapper};
use wasmbus_rpc::{
    actor::prelude::{RpcError, RpcResult},
    common::Context,
};

impl Balance {
    /// Create a new bucket by deserializing a JSON string
    pub fn try_from_str(balance_json_str: &str) -> RpcResult<Balance> {
        serde_json::from_str(balance_json_str).map_err(|e| RpcError::Deser(e.to_string()))
    }

    /// Serialize the bucket to a JSON string
    pub fn serialize(&self) -> RpcResult<String> {
        serde_json::to_string(self).map_err(|e| RpcError::Ser(e.to_string()))
    }

    pub fn balance_count(&self) -> f32 {
        self.balance_characteristic.count
    }

    pub fn balance_unit(&self) -> &str {
        &self.balance_characteristic.unit
    }
}
const BALANCE_BUCKET_NAME: &str = "balance";

#[derive(Clone, Debug, Default)]
pub struct BalanceAccessManager {}

impl BalanceAccessManager {
    fn get_key(&self, customer_id: &str, offer_id: &str) -> String {
        let balance_key = format!("{}:{}:{}", BALANCE_BUCKET_NAME, &customer_id, offer_id);

        balance_key
    }

    async fn put_to_store(&self, _ctx: &Context, key: &str, balance: Balance) -> RpcResult<()> {
        let serialized_balance = balance.serialize()?;
        KeyValueStoreWrapper::put(_ctx, key, &serialized_balance).await?;
        Ok(())
    }

    async fn get_from_store(&self, _ctx: &Context, key: &str) -> RpcResult<Balance> {
        let balance_json_str = KeyValueStoreWrapper::get(_ctx, key).await?;
        let balance: Balance = Balance::try_from_str(&balance_json_str)?;
        Ok(balance)
    }

    pub async fn get_balance(
        &self,
        _ctx: &Context,
        customer_id: &str,
        offer_id: &str,
    ) -> RpcResult<Balance> {
        let balance_key: String = self.get_key(customer_id, offer_id);
        let balance: Balance = self.get_from_store(_ctx, &balance_key).await?;
        Ok(balance)
    }

    pub async fn withdraw(
        &self,
        _ctx: &Context,
        customer_id: &str,
        offer_id: &str,
        amount: f32,
    ) -> RpcResult<Balance> {
        let balance_key: String = self.get_key(customer_id, offer_id);

        let mut balance: Balance = self.get_balance(_ctx, customer_id, offer_id).await?;

        // here we can add any business validations
        balance.balance_characteristic.count -= amount;

        self.put_to_store(_ctx, &balance_key, balance.clone())
            .await?;

        Ok(balance)
    }

}
