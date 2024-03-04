use rating_interface::{
    AgentList, BalanceAccessManager, GetChildrenRequest, KeyValueStoreWrapper,
     RatingAgent, RatingAgentReceiver, RatingRequest, RatingResponse, 
     RatingResponseBuilder, ValidationRequest, ValidationResponse, DeductBalance
};
use serde_json::json;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::info;
use wasmcloud_interface_messaging::{Messaging, MessagingSender, PubMessage};

const UNIT_COST: u32 = 2;
const OFFER_ID: &str = "4";
const BALANCE_COMMANDS_TOPIC: &str = "cc.commands.balance";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, RatingAgent)]
struct PrepaidOrangeVodOneshotAgentActor {}

#[async_trait]
impl RatingAgent for PrepaidOrangeVodOneshotAgentActor {
    async fn rate_usage(&self, _ctx: &Context, _arg: &RatingRequest) -> RpcResult<RatingResponse> {
        info!("Hello I'm your orange prepaid vod One Shot cqrs way agent");

        let mut usage: u32 = 0;
        if let Some(first) = _arg.usage.usage_characteristic_list.first() {
            usage = first.value.parse::<u32>().unwrap();
        }

        // let usage: f32 = _arg.usage.parse().unwrap();
        let customer_id = &_arg.customer_id;

        let mut rating_response_builder = RatingResponseBuilder::new();
        let balance_access_manager = BalanceAccessManager::default();

        let balance_key = format!("balance.{customer_id}");
        let balance_value = KeyValueStoreWrapper::get(_ctx, &balance_key).await?.parse::<u32>().unwrap_or(0);

        let rating = calc_rate(usage);

        // not depending on the balance <=0  but calc rate and keep sufficient
        // balance in has_sufficient_balance to centralize the decision..
        // may be we have a case the customer has 0 balance but he still can use the service ...... [to be validated]

        if has_sufficient_balance(balance_value, rating) {
            info!(
                "Usage {} , Rating {} , & balance {}",
                usage, rating, balance_value
            );

            let data = json!({
                "amount": rating as i64,
                "party_id": customer_id
            });

            let deduct_balance_command = DeductBalance {
                command_type: "DeductBalance".to_string(),
                key: customer_id.to_string(),
                data: data.to_string(),
            };

            let provider = MessagingSender::new();
            provider.publish(
                _ctx,
                &PubMessage {
                    body: serde_json::to_vec(&deduct_balance_command).unwrap(),
                    reply_to: None,
                    subject: BALANCE_COMMANDS_TOPIC.to_owned(),
                },
            ).await;

            let new_balance_value = KeyValueStoreWrapper::get(_ctx, &balance_key).await?.parse::<u32>().unwrap_or(0);

            rating_response_builder
                .unit((&"EUR").to_string())
                .price(rating.to_string())
                .message(&format!(
                    " {} will be deducted from your balance",
                    rating.to_string()
                ))
                .message(&format!(
                    " Your Balance now is {} {}",
                    new_balance_value,
                    "EUR"
                ))
                .authorized();

            info!(
                "Usage {} , Rating {} , & new balance {}",
                usage, rating, new_balance_value
            );
        } else {
            rating_response_builder
                .message(&"You have no sufficient balance")
                .not_authorized();
        }
        let rating_response = rating_response_builder.build();

        RpcResult::Ok(rating_response)
    }

    async fn validate(
        &self,
        ctx: &Context,
        arg: &ValidationRequest,
    ) -> RpcResult<ValidationResponse> {
        let mut validation_response: ValidationResponse = ValidationResponse::default();

        validation_response.valid = true;

        Ok(validation_response)
    }

    async fn get_children(&self, ctx: &Context, arg: &GetChildrenRequest) -> RpcResult<AgentList> {
        Ok(AgentList::new())
    }
}

fn has_sufficient_balance(balance: u32, charge: u32) -> bool {
    if balance < charge {
        return false;
    } else {
        return true;
    }
}

fn calc_rate(usage: u32) -> u32 {
    UNIT_COST * usage
}
