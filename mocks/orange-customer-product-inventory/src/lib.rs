use wasmbus_rpc::actor::prelude::*;
use rating_interface::{
    MockAgent, MockAgentReceiver, DataItem, CustomerInventoryAgent, CustomerInventoryAgentReceiver, Customer
};
use wasmcloud_interface_keyvalue::{SetRequest, KeyValue, KeyValueSender};
use serde_json::json;
use wasmcloud_interface_logging::info;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, MockAgent, CustomerInventoryAgent)]
struct OrangeCustomerProductInventoryActor {}

const OFFERS_LIST_KEY_PREFIX: &str = "inventory";

/// Implementation of Customer Invetory Mock trait methods
#[async_trait]
impl CustomerInventoryAgent for OrangeCustomerProductInventoryActor {
    async fn get_customer<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<Customer> {
        info!("Hello Hello");

        info!("Getting cutomer details for customer: {}", arg.to_string());

        let kv = KeyValueSender::new();

        let customer = kv.get(ctx, arg).await?.value;
        
        Ok(
            Customer {
                value: customer
            }
        )
    }
}

/// Implementation of Data Seeding Mock trait methods
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
        todo!()
    }
}

async fn handle_seeding(ctx: &Context) {
    let data = json!({
           "relatedParty":{
              "id":"12",
              "name":"Jack Black"
           },
           "product":{
              "partnerId":"Netflix",
              "id":1234,
              "description":"”Video on Demand”",
              "productOffering":{
                 "id":1,
                 "name": "Netflix Video On Demand",
                 "agentId": "netflix-vod"
              },
              "productPrice":{
                 "description":"Cost per movie",
                 "validTill":"1/March/2024",
                 "priceType":"non-recurring",
                 "unitOfMeasure":{
                    "amount":"1",
                    "units":"movie-count"
                 },
                 "price":{
                    "unit":"eur",
                    "value":"1"
                 }
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

