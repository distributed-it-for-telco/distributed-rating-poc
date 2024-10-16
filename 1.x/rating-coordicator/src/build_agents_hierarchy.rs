use std::io;
use async_recursion::async_recursion;
use crate::wasi::logging::logging::log;
use crate::wasi::logging::logging::Level::Info;
use crate::orange::rating::types::{
    Agent, AgentIdentification, AgentList, GetChildrenRequest, RatingRequest
};
use crate::wasmcloud::bus::lattice;
// use crate::orange::rating::types::{RatingRequest, RatingResponse};
use crate::orange::rating::*;
use crate::agent_graph::AgentGraph;

pub async fn build_agent_hierarchy(
  
    rating_request: &RatingRequest,
) -> Result<AgentGraph,io::Error> {
    let mut agent_graph = AgentGraph::new();
    log(Info, "", "Add Root to Graph ......");
    let root: Agent = Agent {
        identification: AgentIdentification {
            name: rating_request.agent_id.to_owned(),
            partner_id: rating_request.customer_id.to_owned(),
        },
        usage: rating_request.usage.to_owned(),
    };

    agent_graph.add_vertex(root.clone());
    agent_graph.set_start_vertex(root.clone());
    attach_children( &rating_request, &mut agent_graph, root).await?;

    Ok(agent_graph)
}

#[async_recursion]
pub async fn attach_children(
    rating_request: &RatingRequest,
    graph: &mut AgentGraph,
    root: Agent,
) -> Result<(),io::Error> {
    log(Info, "", format!("Add Chilren to Graph  for {} ......",root.identification.name.to_string()).as_str());
    let children = get_agent_children(&rating_request).await;

    for child in children.agents {
        log(Info, "", format!("Add child {}", child.identification.name.to_string()).as_str());
        graph.add_vertex(child.clone());
        graph.add_edge(root.clone(), child.clone());
        let mut child_rating_request = rating_request.clone();
        child_rating_request.agent_id = child.clone().identification.name;
        child_rating_request.customer_id = child.clone().identification.partner_id;
        child_rating_request.usage = child.clone().usage.clone();
        attach_children(&child_rating_request, graph, child.clone()).await?;
    }

    Ok(())
}

async fn get_agent_children(rating_request: &RatingRequest) -> AgentList {

    let rating_interface = lattice::CallTargetInterface::new(
        "orange",
        "rating",
        "ratingagent",
    );
    lattice::set_link_name(&rating_request.agent_id.to_string(), vec![rating_interface]);

    // let rating_agent: ratingagent = ;

    let children_request = GetChildrenRequest {
        usage: rating_request.usage.to_owned(),
        atomic_offer_id: rating_request.offer_id.to_owned(),
    };

    let rating_interface = lattice::CallTargetInterface::new(
        "orange",
        "rating",
        "ratingagent",
    );
    lattice::set_link_name(&rating_request.agent_id.to_string(), vec![rating_interface]);

    let children = ratingagent::get_children(&children_request);

    return children;
}
