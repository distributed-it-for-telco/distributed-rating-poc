
use async_trait::async_trait;

use rating_interface::{Balance, KeyValueStoreWrapper};
use wasmbus_rpc::{
    actor::prelude::{Actor, HealthResponder, ActorReceiver,RpcError, RpcResult},
    common::Context,
};
use wasmbus_rpc::actor::prelude::*;
use wasmbus_rpc::core::provider::prelude::services;




#[derive(Debug, Default, Actor,HealthResponder)]
#[services(Actor, BalanceActor)]
pub struct BalanceManagerActor {}

const BALANCE_BUCKET_NAME: &str = "balance";

#[async_trait]
impl BalanceManager for  BalanceManagerActor {

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

    async fn get_balance(
        &self,
        _ctx: &Context,
        customer_id: &str,
        offer_id: &str,
    ) -> RpcResult<Balance> {
        let balance_key: String = self.get_key(customer_id, offer_id);
        let balance: Balance = self.get_from_store(_ctx, &balance_key).await?;
        Ok(balance)
    }

    async fn withdraw(
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


    async fn deposit(
        &self,
        _ctx: &Context,
        customer_id: &str,
        offer_id: &str,
        amount: f32,
    ) -> RpcResult<Balance> {
        let balance_key: String = self.get_key(customer_id, offer_id);

        let mut balance: Balance = self.get_balance(_ctx, customer_id, offer_id).await?;

        // here we can add any business validations
        balance.balance_characteristic.count += amount;

        self.put_to_store(_ctx, &balance_key, balance.clone())
            .await?;

        Ok(balance)
    }


}
