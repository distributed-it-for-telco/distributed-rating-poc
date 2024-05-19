use std::collections::HashMap;

use rating_interface::{
    BalanceManager, BalanceManagerSender, CustomerInventoryAgent, CustomerInventoryAgentSender,
    DepositRequest, MockAgent, MockAgentSender, RatingCoordinator, RatingCoordinatorSender,
    RatingProcessRequest, RatingRequest, UsageCollector, UsageCollectorSender,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

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

        info!("Headers: {:?}", req.header);

        match (req.method.as_ref(), trimmed_path.as_slice()) {
            ("OPTIONS", _) => get_options_response(ctx).await,
            ("POST", ["usage", "rating"]) => {
                request_rate(ctx, deser(&req.body)?, extract_headers(ctx, req).await?).await
            }
            ("GET", ["usage", "rating-proofs", usage_collector_id]) => {
                list_usage_proofs(ctx, usage_collector_id).await
            }
            ("POST", ["seed", "orange", "customer", "inventory"]) => {
                seed_data_for_orange_cust_inventory(ctx).await
            }
            ("GET", ["party", party_id, "offers", inventory_agent_id]) => {
                get_party_offers(ctx, party_id, inventory_agent_id).await
            }

            ("POST", ["usage", "balance", "topup"]) => topup_balance(ctx, deser(&req.body)?).await,

            (_, _) => Ok(HttpResponse::not_found()),
        }
    }
}

async fn extract_headers(_ctx: &Context, req: &HttpRequest) -> RpcResult<HashMap<String, String>> {
    let mut headers: HashMap<String, String> = HashMap::new();
    if req.header.contains_key("cf-ipcountry") {
        headers.insert(
            "client_country".to_owned(),
            req.header
                .get("cf-ipcountry")
                .unwrap()
                .get(0)
                .unwrap()
                .to_owned(),
        );
    }

    if req.header.contains_key("cf-connecting-ip") {
        headers.insert(
            "client_ip".to_owned(),
            req.header
                .get("cf-connecting-ip")
                .unwrap()
                .get(0)
                .unwrap()
                .to_owned(),
        );
    }

    info!("Extracted headers {:?}", headers);

    Ok(headers)
}

async fn get_options_response(_ctx: &Context) -> RpcResult<HttpResponse> {
    let mut headers: HashMap<String, Vec<String>> = HashMap::new();
    headers.insert(
        "Access-Control-Allow-Headers".to_owned(),
        vec!["Content-Type, api_key, Authorization".to_string()],
    );
    headers.insert(
        "Access-Control-Allow-Origin".to_owned(),
        vec!["https://editor.swagger.io".to_owned()],
    );
    HttpResponse::json_with_headers("", 204, headers)
}

async fn get_party_offers(
    _ctx: &Context,
    _party_id: &str,
    _inventory_agent_id: &str,
) -> RpcResult<HttpResponse> {
    let customer = CustomerInventoryAgentSender::to_actor(&format!("mock/{}", _inventory_agent_id))
        .get_customer(_ctx, &_party_id.to_lowercase())
        .await?;

    info!("retrieved cutomer details: {:?}", customer.value);

    let customer_inventory_value: Value =
        serde_json::from_str(&customer.value).map_err(|err| RpcError::Ser(format!("{}", err)))?;

    let offers = customer_inventory_value["products"]
        .as_array()
        .unwrap()
        .iter()
        .collect::<Vec<_>>();

    HttpResponse::json_with_headers(offers, 200, get_response_headers())
}

async fn request_rate(
    _ctx: &Context,
    _request: RatingRequest,
    request_headers: HashMap<String, String>,
) -> RpcResult<HttpResponse> {
    let mut rating_process_request: RatingProcessRequest = RatingProcessRequest::default();
    rating_process_request.headers = Some(request_headers);
    rating_process_request.rating_request = _request;

    let rating = RatingCoordinatorSender::to_actor("ratingcoordinator")
        .handle_rating_process(_ctx, &rating_process_request)
        .await?;

    let mut headers: HashMap<String, Vec<String>> = HashMap::new();
    headers.insert(
        "Access-Control-Allow-Headers".to_owned(),
        vec!["Content-Type, api_key, Authorization".to_string()],
    );
    headers.insert(
        "Access-Control-Allow-Origin".to_owned(),
        vec!["https://editor.swagger.io".to_owned()],
    );

    HttpResponse::json_with_headers(rating, 200, headers)
}

async fn seed_data_for_orange_cust_inventory(_ctx: &Context) -> RpcResult<HttpResponse> {
    MockAgentSender::to_actor(&format!("mock/{}", "orange_invetory"))
        .seed(_ctx)
        .await?;

    Ok(HttpResponse::default())
}

async fn list_usage_proofs(ctx: &Context, usage_collector_id: &str) -> RpcResult<HttpResponse> {
    let usage_proof_list = UsageCollectorSender::to_actor(&format!("mock/{}", usage_collector_id))
        .list(ctx)
        .await?;

    let res = usage_proof_list
        .iter()
        .filter_map(|s| match serde_json::from_str(&s.value.as_str()) {
            Ok(v) => Some(v),
            Err(_) => None,
        })
        .collect::<Vec<Value>>();

    HttpResponse::json_with_headers(res, 200, get_response_headers())
}

fn deser<'de, T: Deserialize<'de>>(raw: &'de [u8]) -> RpcResult<T> {
    serde_json::from_slice(raw).map_err(|e| RpcError::Deser(format!("{}", e)))
}

fn get_response_headers() -> HashMap<String, Vec<String>> {
    let mut headers: HashMap<String, Vec<String>> = HashMap::new();
    headers.insert(
        "Access-Control-Allow-Headers".to_owned(),
        vec!["Content-Type, api_key, Authorization".to_string()],
    );
    headers.insert(
        "Access-Control-Allow-Origin".to_owned(),
        vec!["https://editor.swagger.io".to_owned()],
    );
    headers
}

async fn topup_balance(_ctx: &Context, _request: DepositRequest) -> RpcResult<HttpResponse> {
    let balance = BalanceManagerSender::to_actor("balancemanager")
        .deposit(_ctx, &_request)
        .await?;

    let mut headers: HashMap<String, Vec<String>> = HashMap::new();
    headers.insert(
        "Access-Control-Allow-Headers".to_owned(),
        vec!["Content-Type, api_key, Authorization".to_string()],
    );
    headers.insert(
        "Access-Control-Allow-Origin".to_owned(),
        vec!["https://editor.swagger.io".to_owned()],
    );

    HttpResponse::json_with_headers(balance, 200, headers)
}
