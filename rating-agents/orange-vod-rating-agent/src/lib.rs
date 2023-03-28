use std::{collections::HashMap};

use new_string_template::template::Template;
use rating_interface::{
    AuthorizationStatus, BillingInformation, RatingAgent, RatingAgentReceiver,
    RatingRequest, RatingResponse, UsageCollectorSender, UsageCollector
};
use serde::Serialize;
use serde_json::json;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::info;
use wasmcloud_interface_numbergen::{generate_guid};
// use chrono::*;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, RatingAgent)]
struct OrangeVodRatingAgentActor {}

const USAGE_TEMPLATE_PATH : &str = "./usage_proof_template.json";

#[derive(Serialize)]
struct UsageProofContext {
    usage_id: String,
    usage_date: String,
    rating_date: String,
    price: String,
    party_id: String,
    rated_entity_count: String
}

/// Implementation of Rating Agent trait methods
#[async_trait]
impl RatingAgent for OrangeVodRatingAgentActor {
    async fn rate_usage(&self, _ctx: &Context, _arg: &RatingRequest) -> RpcResult<RatingResponse> {
        info!("Hello I'm your orange postpaid vod rating agent");

        // let usage_date = Utc::now().naive_local();
        let usage_date = "";
        let usage_id: String = generate_guid().await?;

        /*
         *  Contract or Offer is one Movie equal one EURO
         */
        let rating = _arg.usage.parse::<i32>().unwrap() * 1;       
        // let rating_date = Utc::now().naive_local();
        let rating_date = "";
       
        // let usage_template_str = std::fs::read_to_string(&USAGE_TEMPLATE_PATH).unwrap();
        let usage_template_str = "{\r\n            \"id\": \"{usage_id}\",\r\n            \"usageDate\": \"{usage_date}\",\r\n            \"description\":\"Video on Demand\",\r\n            \"usageType\":\"VoD\",\r\n            \"ratedProductUsage\":{\r\n               \"isBilled\":false,\r\n               \"ratingAmountType\":\"Total\",\r\n               \"ratingDate\":\"{rating_date}\",\r\n               \"bucketValueConvertedInAmount\":{\r\n                  \"unit\":\"EUR\",\r\n                  \"value\": \"{price}\"\r\n               },\r\n               \"productRef\":{\r\n                  \"id\":\"1234\",\r\n                  \"name\":\"Video on Demand\"\r\n               }\r\n            },\r\n            \"relatedParty\":{\r\n               \"id\":\"{party_id}\"\r\n            },\r\n            \"usageCharacteristic\":[\r\n               {\r\n                  \"id\":\"122\",\r\n                  \"name\":\"movie-count\",\r\n                  \"valueType\":\"integer\",\r\n                  \"value\":\"{rated_entity_count}\"\r\n               }\r\n            ]\r\n         }";
        info!("Before using template engine");

        // let usage_templ = Template::new("Hello {param}");

        let data = {
            let mut map = serde_json::Map::new();
            map.insert("usage_id".into(), usage_id.into());
            map.insert("usage_date".into(), usage_date.into());
            map.insert("rating_date".into(), rating_date.into());
            map.insert("price".into(), rating.into());
            map.insert("party_id".into(), _arg.customer_id.to_string().into());
            map.insert("rated_entity_count".into(), _arg.usage.to_string().into());
            map
        };
        
        // let rendered_templ = usage_templ.render(&data).expect("Expected Result to be Ok");
          
        info!("Starting to send usage proof to usage actor for party with id: {}", _arg.customer_id.to_string());
        // info!("Usage proof: {}", rendered_templ.to_string());
        
        UsageCollectorSender::to_actor(&format!("mock/{}", "usage_collector"))
            .store(_ctx, &serde_json::to_string(&data).map_err(|e| RpcError::Ser(format!("{}", e)))?)
            .await?;

        /*
        * Empty Response till we decide rating response how it should be 
        */
        RpcResult::Ok(RatingResponse {
            authorization_status: AuthorizationStatus::default(),
            billing_information: BillingInformation::default(),
        })
    }
}
