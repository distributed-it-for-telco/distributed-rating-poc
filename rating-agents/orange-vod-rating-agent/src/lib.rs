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

        let usage_template_str =  json!({
            "id": usage_id,
            "usageDate": usage_date,
            "description": "Video on Demand",
            "usageType": "VoD",
            "ratedProductUsage": {
               "isBilled": false,
               "ratingAmountType": "Total",
               "ratingDate": rating_date,
               "bucketValueConvertedInAmount": {
                  "unit": "EUR",
                  "value": rating
               },
               "productRef": {
                  "id": "1234",
                  "name": "Video on Demand"
               }
            },
            "relatedParty": {
               "id": _arg.customer_id
            },
            "usageCharacteristic": [
               {
                  "id": "122",
                  "name": "movie-count",
                  "valueType": "integer",
                  "value": _arg.usage
               }
            ]
         });
                  
        info!("Sending usage proof to usage collector for party with id: {}", _arg.customer_id.to_string());

        UsageCollectorSender::to_actor(&format!("mock/{}", "usage_collector"))
            .store(_ctx, &usage_template_str)
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
