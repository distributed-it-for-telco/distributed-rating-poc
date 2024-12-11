use std::{collections::HashMap, collections::VecDeque};

use crate::wasi::logging::logging::{log, Level::Info};
use crate::wasmcloud::bus::lattice;

use crate::orange::rating::*;
use crate::orange::commons::types::{RatingRequest, RatingResponse,
                    ValidationRequest, ValidationResponse, Agent};
use crate::orange::commons::rating_response_builder as RatingResponseBuilder;
use crate::orange::commons::error_types::ValidationError;
use crate::agent_graph::AgentGraph;
use crate::wasmcloud::bus::lattice::CallTargetInterface;

pub async fn handle_validation_cycle(
    rating_request: &RatingRequest,
    headers: HashMap<String,String>,
    agents_graph: &AgentGraph,
) -> Result<RatingResponse, ValidationError> {
    if headers.is_empty() {
        return Err(ValidationError{message: "Can't validate client usage, client ip not found".to_string()});
    }

    let mut client_ip = "".to_string();
    let mut client_country = "".to_string();
    if !headers.get(&"client_ip".to_string()).is_none(){
        let client_ip_bytes = headers.get(&"client_ip".to_string()).unwrap();
        if !client_ip_bytes.is_empty() {
            client_ip = client_ip_bytes.clone();
        }
    }
    if !headers.get(&"client_country".to_string()).is_none(){
        let client_country_bytes = headers.get(&"client_country".to_string()).unwrap();
        if !client_country_bytes.is_empty() {
            client_country = client_country_bytes.clone();
        }
    }
    log(Info, "", format!("Validating against agent: {}",rating_request.agent_id).as_str());

    let mut validation_response;
    // let mut rating_response_builder = RatingResponseBuilder::new();
    let mut response_builder_handle = RatingResponseBuilder::create_builder();

    let mut visited: HashMap<String, bool> = HashMap::new();
    for vertex in agents_graph.adjacency_list.keys() {
        visited.insert(vertex.clone(), false);
    }
    let mut queue: VecDeque<Agent> = VecDeque::new();
    let start = agents_graph.get_start_vertex().to_owned().unwrap();
    queue.push_back(start.clone());
    visited.insert(start.clone().identification.clone().name.clone(), true);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        println!("Visited vertex: {:?}", current);

        let mut updated_rating_request = rating_request.clone();
        updated_rating_request.customer_id = current.identification.partner_id.clone();
        updated_rating_request.agent_id = current.identification.name.clone();
        updated_rating_request.usage = current.clone().usage.clone();

        validation_response = validate_through_agent(
            &updated_rating_request,
            client_ip.to_string(),
            client_country.to_string(),
        )
        .await;

        if !validation_response.unwrap().valid {
            response_builder_handle= RatingResponseBuilder::message(response_builder_handle, &"Validation failed");
            response_builder_handle= RatingResponseBuilder::not_authorized(response_builder_handle);
            return Ok(RatingResponseBuilder::build(response_builder_handle));
        }

        if let Some(neighbors) = agents_graph
            .adjacency_list
            .get(&current.identification.name)
        {
            for neighbor in neighbors {
                if !visited[&neighbor.clone().identification.name] {
                    visited.insert(neighbor.clone().identification.clone().name.clone(), true);
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }

    response_builder_handle= RatingResponseBuilder::message(response_builder_handle, &"Valid usage");
    response_builder_handle = RatingResponseBuilder::authorized(response_builder_handle);
    Ok(RatingResponseBuilder::build(response_builder_handle))
}

pub async fn validate_through_agent(
    rating_request: &RatingRequest,
    client_ip: String,
    client_country: String,
) -> Result<ValidationResponse, ValidationError> {
    
    let rating_interface = CallTargetInterface::new(
        "orange",
        "rating",
        "ratingagent",
    );
    lattice::set_link_name(&rating_request.agent_id.to_string(), vec![rating_interface]);
    
    log(Info, "", &rating_request.agent_id.to_string());

    let validation_request = ValidationRequest {
        rating_request: rating_request.to_owned(),
        client_ip: client_ip,
        client_country: client_country,
    };

    let validation_response = ratingagent::validate(&validation_request);

    validation_response
}
