use async_recursion::async_recursion;
use rating_interface::{
    Agent, AgentIdentifiation, AgentList, GetChildrenRequest, RatingAgent, RatingAgentSender,
    RatingProcessRequest, RatingRequest, RatingResponse, RatingResponseBuilder, Usage,
    ValidationRequest, ValidationResponse,
};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::info;

use crate::agent_graph::AgentGraph;

pub async fn build_agent_hierarchy(
    _ctx: &Context,
    rating_request: &RatingRequest,
) -> RpcResult<AgentGraph> {
    let mut agentGraph = AgentGraph::new();

    let mut root: Agent = Agent {
        identifiation: AgentIdentifiation {
            name: rating_request.agent_id.to_owned(),
            partner_id: rating_request.customer_id.to_owned(),
        },
        usage: Some(rating_request.usage.to_owned()),
    };

    agentGraph.add_vertex(root.clone());
    agentGraph.set_start_vertex(root.clone());
    attach_children(&_ctx, &rating_request, &mut agentGraph, root).await?;

    Ok(agentGraph)
}

#[async_recursion]
pub async fn attach_children(
    _ctx: &Context,
    rating_request: &RatingRequest,
    graph: &mut AgentGraph,
    root: Agent,
) -> RpcResult<()> {
    let children = get_agent_children(&_ctx, &rating_request).await?;

    for child in children {
        graph.add_vertex(child.clone());
        graph.add_edge(root.clone(), child.clone());
        let mut child_rating_request = rating_request.clone();
        child_rating_request.usage = child.clone().usage.unwrap().clone();
        attach_children(&_ctx, rating_request, graph, child.clone()).await?;
    }

    Ok(())
}

async fn get_agent_children(ctx: &Context, rating_request: &RatingRequest) -> RpcResult<AgentList> {
    let rating_agent: RatingAgentSender<WasmHost> =
        RatingAgentSender::to_actor(&format!("agent/{}", rating_request.agent_id.to_string()));

    let children_request = GetChildrenRequest {
        usage: rating_request.usage.to_owned(),
        atomic_offer_id: rating_request.offer_id.to_owned(),
    };

    let children = rating_agent.get_children(ctx, &children_request).await?;

    Ok(children)
}
