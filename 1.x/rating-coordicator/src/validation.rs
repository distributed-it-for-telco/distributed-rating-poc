use crate::wasi::logging::logging::{log, Level::Info};
use std::{fmt,collections::HashMap, collections::VecDeque};

use crate::orange::rating::*;
use crate::orange::rating::types::*;
use crate::exports::orange::ratingcoordinator::types::{RatingProcessRequest};

use crate::agent_graph::AgentGraph;
use crate::rating_reponse_builder::{RatingResponseBuilder};
use crate::wasmcloud::bus::lattice::CallTargetInterface;
use crate::wasmcloud::bus::lattice;

pub async fn handle_validation_cycle(
    rating_process_request: &RatingProcessRequest,
    agents_graph: &AgentGraph,
) -> Result<RatingResponse, ValidationError> {
    if rating_process_request.headers.entries().is_empty() {
        return Err(ValidationError::Other("Can't validate client usage, client ip not found".to_string()));
    }

    let client_headers = &rating_process_request.headers;
    let mut client_ip = "".to_string();
    let mut client_country = "".to_string();

    let client_ip_bytes = client_headers.get(&"client_ip".to_string());
    if !client_ip_bytes.is_empty() {
        client_ip = String::from_utf8(client_ip_bytes[0].clone()).unwrap();
    }
    let client_country_bytes = client_headers.get(&"client_country".to_string());
    if !client_country_bytes.is_empty() {
        client_country = String::from_utf8(client_country_bytes[0].clone()).unwrap();
    }

    log(Info, "", format!("Validating against agent: {}",rating_process_request.rating_request.agent_id).as_str());

    let mut validation_response;
    let mut rating_response_builder = RatingResponseBuilder::new();

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

        let mut updated_rating_request = rating_process_request.rating_request.clone();
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
            rating_response_builder
                .message(&"Validation failed")
                .not_authorized();
            return Ok(rating_response_builder.build());
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

    rating_response_builder.message(&"Valid usage").authorized();
    Ok(rating_response_builder.build())
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

    let validation_response = rating_agent::validate(&validation_request).await?;

    Ok(validation_response)
}

#[derive(Debug)]
enum ValidationError {
    Other(String), // For generic errors
}
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::Other(message) => write!(f, "An error occurred: {}", message),
        }
    }
}
