use rating_interface::{
    AuthorizationStatus, BillingInformation, RatingAgent, RatingAgentReceiver, RatingRequest,
    RatingResponse, UsageCollector, UsageCollectorSender,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_keyvalue::{KeyValue, KeyValueSender, SetRequest};
use wasmcloud_interface_logging::info;
use wasmcloud_interface_numbergen::generate_guid;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, RatingAgent)]
struct PrepaidOrangeVodOneshotAgentActor {}

#[derive(Serialize, Deserialize, Debug)]
struct Balance {
    #[serde(rename = "partyId")]
    party_id: String,
    #[serde(rename = "balanceCharacteristic")]
    balance_characteristic: BalanceCharacteristic,
}

#[derive(Serialize, Deserialize, Debug)]
struct BalanceCharacteristic {
    #[serde(rename = "unit")]
    unit: String,
    #[serde(rename = "count")]
    pub count: f64,
}

const BALANCE_KEY_PREFIX: &str = "balance";

const OFFER_ID: &str = "4";

const UNIT_COST: f64 = 2.0;

/// Implementation of HttpServer trait methods
#[async_trait]
impl RatingAgent for PrepaidOrangeVodOneshotAgentActor {
    /// Returns a greeting, "Hello World", in the response body.
    /// If the request contains a query parameter 'name=NAME', the
    /// response is changed to "Hello NAME"
    async fn rate_usage(&self, _ctx: &Context, _arg: &RatingRequest) -> RpcResult<RatingResponse> {
        info!("Hello I'm your orange prepaid vod One Shot agent");
        let balance_key = format!(
            "{}:{}:{}",
            BALANCE_KEY_PREFIX,
            &_arg.customer_id.as_str(),
            OFFER_ID
        );
        let balance = get_party_balance(_ctx, balance_key.as_str()).await?;

        let mut billing_info = BillingInformation::default();
        let mut authorization_status = AuthorizationStatus::default();

        if balance.balance_characteristic.count <= 0.0 {
            billing_info
                .messages
                .push(format!("You have no sufficient balance"));
            authorization_status.code = 401;
        } else {
            let usage: f64 = _arg.usage.parse().unwrap();
            let rating = UNIT_COST * usage;
            info!(
                "Usage {} , Rating {} , & balance {}",
                usage, rating, balance.balance_characteristic.count
            );
            if balance.balance_characteristic.count < rating {
                billing_info
                    .messages
                    .push(format!("You have no sufficient balance"));
                authorization_status.code = 401;
            } else {
                info!("decrease_balance");

                decrease_balance(_ctx, &balance_key, rating).await?;

                billing_info.unit = (&"EUR").to_string();
                billing_info.price = rating.to_string();
                billing_info.messages.push(format!(
                    " {} will be deducted from your balance",
                    rating.to_string()
                ));
                let new_balance = get_party_balance(_ctx, balance_key.as_str()).await?;
                info!(
                    "Usage {} , Rating {} , & balance {}",
                    usage, rating, new_balance.balance_characteristic.count
                );
                billing_info.messages.push(format!(
                    " Your Balance now is {} {}",
                    new_balance.balance_characteristic.count,
                    new_balance.balance_characteristic.unit
                ));
            }
        }

        RpcResult::Ok(RatingResponse {
            authorization_status: authorization_status,
            billing_information: billing_info,
        })
    }
}

async fn get_party_balance(_ctx: &Context, balance_key: &str) -> RpcResult<Balance> {
    let kv = KeyValueSender::new();
    let balance_json_str = kv.get(_ctx, balance_key).await?.value;
    let balance: Balance =
        serde_json::from_str(&balance_json_str).map_err(|err| RpcError::Ser(format!("{}", err)))?;

    Ok(balance)
}

async fn decrease_balance(_ctx: &Context, balance_key: &str, rate: f64) -> RpcResult<()> {
    info!("New Balance........................");
    let kv = KeyValueSender::new();

    let balance_json_str = kv.get(_ctx, &balance_key).await?.value;

    let mut balance: Balance =
        serde_json::from_str(&balance_json_str).map_err(|err| RpcError::Ser(format!("{}", err)))?;

    balance.balance_characteristic.count -= rate;

    let serialized_balance =
        serde_json::to_string(&balance).map_err(|err| RpcError::Ser(format!("{}", err)))?;

    kv.set(
        _ctx,
        &SetRequest {
            key: balance_key.to_string(),
            value: serialized_balance,
            expires: 0,
        },
    )
    .await?;

    Ok(())
}
