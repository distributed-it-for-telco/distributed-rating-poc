use std::time::Instant;

use rating_interface::{
    AuthorizationStatus, BillingInformation, RatingAgent, RatingAgentReceiver, RatingAgentSender,
    RatingRequest, RatingResponse, UsageCollectorSender, UsageCollector
};
use serde::Serialize;
use tinytemplate::TinyTemplate;
use uuid::Uuid;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::info;
use chrono::*;

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

        let usage_date = Utc::now().naive_local();
        let id = Uuid::new_v4();

        let usage_template = std::fs::read_to_string(&USAGE_TEMPLATE_PATH).unwrap();
        let rating = _arg.usage.parse::<i32>().unwrap() * 1;       
        let rating_date = Utc::now().naive_local();

        // let mut tiny = TinyTemplate::new();
        // tt.add_template("usage_proof_template", &usage_template).unwrap();

        let context = UsageProofContext {
            usage_id: id.to_string(),
            usage_date: usage_date.to_string(),
            rating_date: rating_date.to_string(),
            price: rating.to_string(),
            party_id: _arg.customer_id.to_string(),
            rated_entity_count: _arg.usage.to_string()
        };

        // let rendered_usage = tt.render("usage_proof_template", &context).unwrap();

        let test = UsageCollectorSender::to_actor(&format!("mock/{}", "usage_collector"))
            .store(_ctx, "")
            .await?;

        RpcResult::Ok(RatingResponse {
            authorization_status: AuthorizationStatus::default(),
            billing_information: BillingInformation::default(),
        })
    }
}
