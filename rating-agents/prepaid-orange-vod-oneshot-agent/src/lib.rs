use rating_interface::{
    AuthorizationStatus, BillingInformation, RatingAgent, RatingAgentReceiver, RatingRequest,
    RatingResponse, UsageCollector, UsageCollectorSender,
};
use serde::{Serialize, Deserialize};
use serde_json::{json};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_keyvalue::{KeyValueSender, KeyValue, SetRequest};
use wasmcloud_interface_logging::info;
use wasmcloud_interface_numbergen::generate_guid;


#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, RatingAgent)]
struct PrepaidOrangeVodOneshotAgentActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl RatingAgent for PrepaidOrangeVodOneshotAgentActor {
    /// Returns a greeting, "Hello World", in the response body.
    /// If the request contains a query parameter 'name=NAME', the
    /// response is changed to "Hello NAME"
    async fn rate_usage(&self, _ctx: &Context, _arg: &RatingRequest) -> RpcResult<RatingResponse> {

        RpcResult::Ok(RatingResponse {
            authorization_status: AuthorizationStatus::default(),
            billing_information: BillingInformation::default(),
        })
    }
}

