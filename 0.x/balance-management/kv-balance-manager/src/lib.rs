use async_trait::async_trait;

use rating_interface::{
    Balance, BalanceManager, BalanceManagerReceiver, DepositRequest, KeyValueStoreWrapper,
};
use wasmbus_rpc::{
    actor::prelude::{Actor, ActorReceiver, HealthResponder, RpcResult},
    common::Context,
};

use wasmcloud_interface_logging::info;

const BALANCE_BUCKET_NAME: &str = "balance";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, BalanceManager)]
pub struct BalanceManagerActor {}

impl BalanceManagerActor {
    pub fn get_key(&self, customer_id: &str, offer_id: &str) -> String {
        let balance_key = format!("{}:{}:{}", BALANCE_BUCKET_NAME, customer_id, offer_id);

        balance_key
    }

    pub async fn put_to_store(&self, _ctx: &Context, key: &str, balance: Balance) -> RpcResult<()> {
        let serialized_balance = balance.serialize()?;
        KeyValueStoreWrapper::put(_ctx, key, &serialized_balance).await?;
        Ok(())
    }

    pub async fn get_from_store(&self, _ctx: &Context, key: &str) -> RpcResult<Balance> {
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

    // pub async fn withdraw(
    //     &self,
    //     _ctx: &Context,
    //     customer_id: &str,
    //     offer_id: &str,
    //     amount: f32,
    // ) -> RpcResult<Balance> {
    //     let balance_key: String = self.get_key(customer_id, offer_id);

    //     let mut balance: Balance = self.get_balance(_ctx, customer_id, offer_id).await?;

    //     // here we can add any business validations
    //     balance.balance_characteristic.count -= amount;

    //     self.put_to_store(_ctx, &balance_key, balance.clone())
    //         .await?;

    //     Ok(balance)
    // }
}

#[async_trait]
impl BalanceManager for BalanceManagerActor {
    async fn deposit(&self, _ctx: &Context, deposit_request: &DepositRequest) -> RpcResult<Balance> {

        let balance_key: String = self.get_key(deposit_request.customer_id.as_str(), deposit_request.offer_id.as_str());
        
        info!("Balance key to be deposited: {:?}", balance_key);
        
        let mut balance: Balance = self
            .get_balance(
                _ctx,
                deposit_request.customer_id.as_str(),
                deposit_request.offer_id.as_str(),
            )
            .await?;

        // here we can add any business validations
        balance.balance_characteristic.count += deposit_request.amount;

        self.put_to_store(_ctx, &balance_key, balance.clone())
            .await?;

        Ok(balance)
    }
}
