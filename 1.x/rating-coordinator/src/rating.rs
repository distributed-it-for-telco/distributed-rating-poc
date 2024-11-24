use std::collections::HashMap;

use crate::wasi::logging::logging::{log, Level::Info};
use crate::wasmcloud::bus::lattice;

use crate::orange::rating::*;
use crate::orange::commons::types::*;
use crate::agent_graph::AgentGraph;
use async_recursion::async_recursion;

pub async fn handle_rating_cycle(
    rating_request: &RatingRequest,
    agents_graph: &AgentGraph,
) -> RatingResponse {
    let mut visited: HashMap<String, bool> = HashMap::new();
    for vertex in agents_graph.adjacency_list.keys() {
        visited.insert(vertex.to_string(), false);
    }
    dfs_recursive(
        agents_graph.get_start_vertex().to_owned().unwrap().clone(),
        &visited,
        &agents_graph,
        &mut rating_request.clone(),
    )
    .await
}

pub async fn rate_through_agent(
    rating_request: &RatingRequest,
) -> RatingResponse {
    let rating_interface = lattice::CallTargetInterface::new(
        "orange",
        "rating",
        "ratingagent",
    );
    lattice::set_link_name(&rating_request.agent_id.to_string(), vec![rating_interface]);

    let rating_response = ratingagent::rate_usage(rating_request);

    rating_response
}

#[async_recursion]
async fn dfs_recursive(
    vertex: Agent,
    visited: &HashMap<String, bool>,
    agents_graph: &AgentGraph,
    rating_request: &mut RatingRequest,
) -> RatingResponse {
    let mut children_ratings = Vec::new();
    if let Some(neighbors) = agents_graph.adjacency_list.get(&vertex.identification.name) {
        for neighbor in neighbors {
            if !visited[&neighbor.identification.name] {
                let child_rating = dfs_recursive(
                    neighbor.clone(),
                    visited,
                    &agents_graph,
                    rating_request,
                ).await;

                if child_rating.authorization_status.code == 401 {
                    return child_rating;
                }

                children_ratings.push(child_rating);
            }
        }
    }

    println!("My Rating {:?} ", vertex);
    let current_agent = vertex.clone();
    rating_request.agent_id = current_agent.identification.name;
    rating_request.customer_id = current_agent.identification.partner_id;
    rating_request.usage = current_agent.usage;

     log(Info, "","Prepare rating history .....");
    let mut child_records = Vec::new();
    for child_rating in children_ratings {
        let mut rating_record: RatingRecord = RatingRecord{
            producer: "".to_string(),
            unit: "".to_string(),
            price: "".to_string()
        };
        rating_record.producer = rating_request.agent_id.to_owned();
        rating_record.unit = child_rating.billing_information.unit;
        rating_record.price = child_rating.billing_information.price;
        child_records.push(rating_record);
    }
    rating_request.rating_history = child_records;

    let rating_response = rate_through_agent(&rating_request).await;

    log(Info, "",format!("Provider Rating status: {}",rating_response.authorization_status.code).as_str());

    rating_response
}
