use wasmbus_rpc::actor::prelude::*;
use rating_interface::{
    MockAgent, MockAgentSender, MockAgentReceiver, DataList
};
use wasmcloud_interface_keyvalue::{SetRequest, KeyValue, KeyValueSender};
use serde_json::json;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, MockAgent)]
struct OrangeCustomerProductInventoryActor {}

/// Implementation of Rating Mock trait methods
#[async_trait]
impl MockAgent for OrangeCustomerProductInventoryActor {
    async fn seed(&self, ctx: &Context) -> RpcResult<()> {
        handle_seeding(&ctx);
        Ok(()) 
    }

    async fn get_data_item<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<DataItem> {
        let kv = KeyValueSender::new();

        kv.get(
            ctx,
            &arg
        )
        .await
        .map(|s| {
            match serde_json::from_str(s.as_str()) {
                    Ok(v) => Some(v),
                    Err(_) => None,
                }
        })
        .collect::<DataItem>()  
    }
}

async fn handle_seeding(ctx: &Context) {
    let data = json!({
        "product": {
           "id":1234,
           "offerId":534,
           "description":"”Video on Demand”",
           "productPrice": {
              "description":"”Cost per movie”",
              "validTill":"1/March/2024",
              "priceType":"non-recurring",
              "unitOfMeasure": {
                 "amount":"1",
                 "units":"”movie”"
              },
              "price": {
                 "unit":"eur",
                 "value":"1"
              },
              "ratingAgent": {}
           }
        }
    });
    
    KeyValueSender::new()
        .set(ctx, &SetRequest { key: "1".to_string(), value: data.to_string(), expires: 0 })
        .await;

    KeyValueSender::new()
        .set(ctx, &SetRequest {  key: "2".to_string(), value: data.to_string(), expires: 0 })
        .await;
}

async fn retreive_party
#[cfg(test)]
mod tests {
    use crate::handle_seeding;

    #[tokio::test]
    fn can_handle_seedig() {
        handle_seeding().await;4
    }
}

