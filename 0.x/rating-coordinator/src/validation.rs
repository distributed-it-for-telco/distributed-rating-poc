use std::collections::{HashMap, VecDeque};

use rating_interface::{
    Agent, RatingAgent, RatingAgentSender, RatingProcessRequest, RatingRequest,
    RatingResponse, RatingResponseBuilder, ValidationRequest, ValidationResponse,
};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::info;

use crate::agent_graph::AgentGraph;

pub async fn handle_validation_cycle(
    _ctx: &Context,
    rating_process_request: &RatingProcessRequest,
    agents_graph: &AgentGraph,
) -> RpcResult<RatingResponse> {
    if rating_process_request.headers.is_none() {
        return RpcResult::from(Err(RpcError::Other(
            "Can't validate client usage, client ip not found!".to_owned(),
        )));
    }

    let client_headers = rating_process_request.headers.to_owned().unwrap();
    let mut client_ip = "";
    let mut client_country = None;
    if let Some(extracted_client_ip) = client_headers.get("client_ip") {
        client_ip = extracted_client_ip;
    }

    if let Some(lcl_client_country) = client_headers.get("client_country") {
        client_country = Some(lcl_client_country);
    }

    info!(
        "Validating against agent: {}",
        rating_process_request.rating_request.agent_id
    );

    let mut validation_response ;
    let mut rating_response_builder = RatingResponseBuilder::new();

    let mut visited: HashMap<String, bool> = HashMap::new();
    for vertex in agents_graph.adjacency_list.keys() {
        visited.insert(vertex.clone(), false);
    }
    let mut queue: VecDeque<Agent> = VecDeque::new();
    let start = agents_graph.get_start_vertex().to_owned().unwrap();
    queue.push_back(start.clone());
    visited.insert(start.clone().identifiation.clone().name.clone(), true);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        println!("Visited vertex: {:?}", current);

        let mut updated_rating_request = rating_process_request.rating_request.clone();
        updated_rating_request.customer_id = current.identifiation.partner_id.clone();
        updated_rating_request.agent_id = current.identifiation.name.clone();
        updated_rating_request.usage = current.clone().usage.unwrap().clone();

        validation_response = validate_through_agent(
            _ctx,
            &updated_rating_request,
            client_ip.to_string(),
            client_country.to_owned().cloned(),
        )
        .await?;

        if !validation_response.valid {
            rating_response_builder
                .message(&"Validation failed")
                .not_authorized();
            return Ok(rating_response_builder.build());
        }

        if let Some(neighbors) = agents_graph.adjacency_list.get(&current.identifiation.name) {
            for neighbor in neighbors {
                if !visited[&neighbor.clone().identifiation.name] {
                    visited.insert(neighbor.clone().identifiation.clone().name.clone(), true);
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }

    rating_response_builder.message(&"Valid usage").authorized();
    Ok(rating_response_builder.build())
}

pub async fn validate_through_agent(
    ctx: &Context,
    rating_request: &RatingRequest,
    client_ip: String,
    client_country: Option<String>,
) -> RpcResult<ValidationResponse> {
    let rating_agent: RatingAgentSender<WasmHost> =
        RatingAgentSender::to_actor(&format!("agent/{}", rating_request.agent_id.to_string()));

    info!("rating agent: {:?}", rating_agent);

    let validation_request = ValidationRequest {
        rating_request: rating_request.to_owned(),
        client_ip: client_ip,
        client_country: client_country,
    };

    let validation_response = rating_agent.validate(ctx, &validation_request).await?;

    Ok(validation_response)
}
