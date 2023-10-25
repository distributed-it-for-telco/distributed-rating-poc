use std::collections::HashMap;

use async_recursion::async_recursion;
use rating_interface::{
    Agent, RatingAgent, RatingAgentSender, RatingProcessRequest, RatingRecord, RatingRequest,
    RatingResponse, Usage,
};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::info;

use crate::agent_graph::AgentGraph;

pub async fn handle_rating_cycle(
    _ctx: &Context,
    rating_process_request: &RatingProcessRequest,
    agents_graph: &AgentGraph,
) -> RpcResult<RatingResponse> {
    let mut visited: HashMap<String, bool> = HashMap::new();
    for vertex in agents_graph.adjacency_list.keys() {
        visited.insert(vertex.to_string(), false);
    }
    dfs_recursive(
        &_ctx,
        agents_graph.get_start_vertex().to_owned().unwrap().clone(),
        &visited,
        &agents_graph,
        &mut rating_process_request.rating_request.clone(),
    )
    .await
}

pub async fn rate_through_agent(
    ctx: &Context,
    rating_request: &RatingRequest,
) -> RpcResult<RatingResponse> {
    let rating_agent: RatingAgentSender<WasmHost> =
        RatingAgentSender::to_actor(&format!("agent/{}", rating_request.agent_id.to_string()));

    let rating_response = rating_agent.rate_usage(ctx, rating_request).await?;

    Ok(rating_response)
}

#[async_recursion]
async fn dfs_recursive(
    _ctx: &Context,
    vertex: Agent,
    visited: &HashMap<String, bool>,
    agents_graph: &AgentGraph,
    rating_request: &mut RatingRequest,
) -> RpcResult<RatingResponse> {
    let mut children_ratings = Vec::new();
    if let Some(neighbors) = agents_graph.adjacency_list.get(&vertex.identifiation.name) {
        for neighbor in neighbors {
            if !visited[&neighbor.identifiation.name] {
                let child_rating = dfs_recursive(
                    &_ctx,
                    neighbor.clone(),
                    visited,
                    &agents_graph,
                    rating_request,
                )
                .await?;

                if child_rating.authorization_status.code == 401 {
                    return Ok(child_rating);
                }

                children_ratings.push(child_rating);
            }
        }
    }

    println!("My Rating {:?} ", vertex);
    let current_agent = vertex.clone();
    rating_request.agent_id = current_agent.identifiation.name;
    rating_request.customer_id = current_agent.identifiation.partner_id;

    if let Some(usage) = current_agent.usage {
        rating_request.usage = usage;
    }

    info!("Prepare rating history .....");
    let mut child_records = Vec::new();
    for child_rating in children_ratings {
        let mut rating_record: RatingRecord = RatingRecord::default();
        rating_record.producer = rating_request.agent_id.to_owned();
        rating_record.unit = child_rating.billing_information.unit;
        rating_record.price = child_rating.billing_information.price;
        child_records.push(rating_record);
    }
    rating_request.rating_history = Some(child_records);

    let rating_response = rate_through_agent(_ctx, &rating_request).await?;

    info!(
        "Provider Rating status: {}",
        rating_response.authorization_status.code
    );

    Ok(rating_response)
}
