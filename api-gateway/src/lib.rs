use std::collections::HashMap;

use rating_interface::{
    CustomerInventoryAgent, CustomerInventoryAgentSender, MockAgent, MockAgentSender, RatingAgent,
    RatingAgentSender, RatingRequest,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use types::{
    AuthorizationStatus, BillingInformation, RequestApproval, ServiceUsageRatingRequest,
    ServiceUsageRatingResponse, ServiceUsageRequest, ServiceUsageResponse,
};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_logging::info;

mod types;

#[derive(Serialize, Deserialize, Debug)]
struct Party {
    #[serde(rename = "products")]
    products: Vec<Product>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Product {
    #[serde(rename = "partnerId")]
    partner_id: String,
    #[serde(rename = "id")]
    id: u32,
}

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct ApiGatewayActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for ApiGatewayActor {
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        let trimmed_path: Vec<&str> = req.path.trim_matches('/').split('/').collect();
        info!("Trimmed Path: {:?}", trimmed_path);

        match (req.method.as_ref(), trimmed_path.as_slice()) {
            ("OPTIONS", _) => get_options_response(ctx).await,
            ("POST", ["usage", "rating"]) => request_rate(ctx, deser(&req.body)?).await,
            ("POST", ["usage", "requests"]) => request_usage(ctx, deser(&req.body)?).await,
            ("POST", ["usage", "rating_events"]) => {
                record_rated_usage(ctx, deser(&req.body)?).await
            }
            ("POST", ["seed", "orange", "customer", "inventory"]) => {
                seed_data_for_orange_cust_inventory(ctx).await
            }
            ("GET", ["party", party_id, "offers", vendor]) => {
                get_party_offers(ctx, party_id, vendor).await
            }
            (_, _) => Ok(HttpResponse::not_found()),
        }
    }
}

async fn get_options_response(
    _ctx: &Context
) -> RpcResult<HttpResponse> {
        
    let mut headers: HashMap<String, Vec<String>> = HashMap::new();
    headers.insert("Access-Control-Allow-Headers".to_owned(), vec!["Content-Type, api_key, Authorization".to_string()]);
    headers.insert("Access-Control-Allow-Origin".to_owned(), vec!["https://editor.swagger.io".to_owned()]);
    HttpResponse::json_with_headers("", 204, headers)
}

async fn get_party_offers(
    _ctx: &Context,
    _party_id: &str,
    _vendor: &str,
) -> RpcResult<HttpResponse> {
    let customer = CustomerInventoryAgentSender::to_actor(&format!("mock/{}", "orange_inventory"))
        .get_customer(_ctx, &_party_id)
        .await?;

    info!("retrieved cutomer details: {:?}", customer.value);

    let customer_inventory_value: Value =
        serde_json::from_str(&customer.value).map_err(|err| RpcError::Ser(format!("{}", err)))?;

    let offers = customer_inventory_value["products"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|product| product["partnerId"] == _vendor)
        .collect::<Vec<_>>();

    let mut headers: HashMap<String, Vec<String>> = HashMap::new();
    headers.insert("Access-Control-Allow-Headers".to_owned(), vec!["Content-Type, api_key, Authorization".to_string()]);
    headers.insert("Access-Control-Allow-Origin".to_owned(), vec!["https://editor.swagger.io".to_owned()]);

    HttpResponse::json_with_headers(offers, 200, headers)
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

    let mut headers: HashMap<String, Vec<String>> = HashMap::new();
    headers.insert("Access-Control-Allow-Headers".to_owned(), vec!["Content-Type, api_key, Authorization".to_string()]);
    headers.insert("Access-Control-Allow-Origin".to_owned(), vec!["https://editor.swagger.io".to_owned()]);

    HttpResponse::json_with_headers(rating, 200, headers)
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
