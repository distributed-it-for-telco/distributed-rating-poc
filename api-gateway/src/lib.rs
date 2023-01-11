use types::{ServiceUsageRequest, ServiceUsageRatingRequest, ServiceUsageResponse, AuthorizationStatus, BillingInformation, RequestApproval, ServiceUsageRatingResponse};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use serde::{Serialize, Deserialize};

mod types;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct ApiGatewayActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for ApiGatewayActor {
    
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        let trimmed_path: Vec<&str> = req.path.trim_matches('/').split('/').collect();

        match (req.method.as_ref(), trimmed_path.as_slice()) {
            ("POST", ["usage", "requests"]) => request_usage(ctx, deser(&req.body)?).await,
            ("POST", ["usage", "rating_events"]) => record_rated_usage(ctx, deser(&req.body)?).await,
            (_, _) => Ok(HttpResponse::not_found())
        }
    }
}

async fn request_usage(_ctx: &Context, _request: ServiceUsageRequest) -> RpcResult<HttpResponse> {
    // TODO: make request of backend to get this data
    let resp = ServiceUsageResponse {
        authorization_status: AuthorizationStatus::default(),
        billing_information: BillingInformation::default(),
        request_approvals: vec![
            RequestApproval::default()
        ],
    };
    HttpResponse::json(resp, 200)
}

async fn record_rated_usage(_ctx: &Context, _request: ServiceUsageRatingRequest) -> RpcResult<HttpResponse> {
    // TODO: make request of backend to get this data
    let resp = ServiceUsageRatingResponse {
        authorization_status: AuthorizationStatus::default(),
        billing_information: BillingInformation::default()
    };
    HttpResponse::json(resp, 200)
}


fn deser<'de, T: Deserialize<'de>>(raw: &'de [u8]) -> RpcResult<T> {
    serde_json::from_slice(raw).map_err(|e| RpcError::Deser(format!("{}", e)))
}

