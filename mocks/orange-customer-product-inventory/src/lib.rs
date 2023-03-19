use wasmbus_rpc::actor::prelude::*;
use rating_interface::{
    MockAgent, MockAgentSender, MockAgentReceiver, OffersList, DataItem, CustomerInventoryAgent, ListOffersRequest
};
use wasmcloud_interface_keyvalue::{SetRequest, KeyValue, KeyValueSender};
use serde_json::json;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, MockAgent)]
struct OrangeCustomerProductInventoryActor {}

/// Implementation of Customer Invetory Mock trait methods
#[async_trait]
impl CustomerInventoryAgent for OrangeCustomerProductInventoryActor {
    async fn get_customer_offers(
        &self,
        ctx: &Context,
        arg: &ListOffersRequest,
    ) -> RpcResult<OffersList> {
        todo!()
    }
}

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

