use rating_interface::{RatingAgent, RatingAgentSender, RatingRequest, MockAgentSender, MockAgent};
use serde::Deserialize;
use types::{
    AuthorizationStatus, BillingInformation, RequestApproval, ServiceUsageRatingRequest,
    ServiceUsageRatingResponse, ServiceUsageRequest, ServiceUsageResponse,
};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_logging::{info};

mod types;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct ApiGatewayActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for ApiGatewayActor {
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        let trimmed_path: Vec<&str> = req.path.trim_matches('/').split('/').collect();
        info!("This is an info level log!");
        info!("Trimmed Path: {:?}", trimmed_path);

        match (req.method.as_ref(), trimmed_path.as_slice()) {
            ("POST", ["usage", "rating"]) => request_rate(ctx, deser(&req.body)?).await,
            ("POST", ["usage", "requests"]) => request_usage(ctx, deser(&req.body)?).await,
            ("POST", ["usage", "rating_events"]) => record_rated_usage(ctx, deser(&req.body)?).await,
            ("POST", ["seed", "orange", "customer", "inventory"]) => seed_data_for_orange_cust_inventory(ctx).await,
            (_, _) => Ok(HttpResponse::not_found()),
        }
    }
}

async fn request_usage(_ctx: &Context, _request: ServiceUsageRequest) -> RpcResult<HttpResponse> {
    // TODO: make request of backend to get this data
    let resp = ServiceUsageResponse {
        authorization_status: AuthorizationStatus::default(),
        billing_information: BillingInformation::default(),
        request_approvals: vec![RequestApproval::default()],
    };
    HttpResponse::json(resp, 200)
}

async fn request_rate(_ctx: &Context, _request: RatingRequest) -> RpcResult<HttpResponse> {
    let rating = RatingAgentSender::to_actor("ratingcoordinator")
        .rate_usage(_ctx, &_request)
        .await?;

    HttpResponse::json(rating, 200)
}

async fn seed_data_for_orange_cust_inventory(_ctx: &Context) -> RpcResult<HttpResponse> {

    MockAgentSender::to_actor(&format!("mock/{}", "orange_invetory"))
        .seed(_ctx)
        .await?;
    
    Ok(HttpResponse::default())
}

async fn record_rated_usage(
    _ctx: &Context,
    _request: ServiceUsageRatingRequest,
) -> RpcResult<HttpResponse> {
    // TODO: make request of backend to get this data
    let resp = ServiceUsageRatingResponse {
        authorization_status: AuthorizationStatus::default(),
        billing_information: BillingInformation::default(),
    };
    HttpResponse::json(resp, 200)
}

fn deser<'de, T: Deserialize<'de>>(raw: &'de [u8]) -> RpcResult<T> {
    serde_json::from_slice(raw).map_err(|e| RpcError::Deser(format!("{}", e)))
}
