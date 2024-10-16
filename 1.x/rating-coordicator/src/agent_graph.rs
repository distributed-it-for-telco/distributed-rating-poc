use crate::orange::rating::types::{ Agent };
use std::collections::HashMap;

pub struct AgentGraph {
    pub adjacency_list: HashMap<String, Vec<Agent>>,
    agents_map: HashMap<String, Agent>,
    start_vertex: Option<Agent>,
}

impl AgentGraph {
    // Create a new empty graph
    pub fn new() -> Self {
        AgentGraph {
            adjacency_list: HashMap::new(),
            agents_map: HashMap::new(),
            start_vertex: None,
        }
    }

    pub fn set_start_vertex(&mut self, start: Agent) {
        self.start_vertex = Some(start);
    }

    // Get the start vertex for traversal
    pub fn get_start_vertex(&self) -> &Option<Agent> {
        &self.start_vertex
    }

    // Add a vertex to the graph
    pub fn add_vertex(&mut self, vertex: Agent) {
        self.adjacency_list
            .entry(vertex.identification.name.clone())
            .or_insert(Vec::new());
        self.agents_map
            .entry(vertex.identification.name.clone())
            .or_insert(vertex);
    }

    // Add an edge between two vertices
    pub fn add_edge(&mut self, from: Agent, to: Agent) {
        self.adjacency_list
            .entry(from.identification.name)
            .or_insert(Vec::new())
            .push(to);
    }

    // Perform depth-first search traversal
    pub fn dfs(&self, start: Agent) {
        let mut visited: HashMap<String, bool> = HashMap::new();
        for vertex in self.adjacency_list.keys() {
            visited.insert(vertex.to_string(), false);
        }
        self.dfs_recursive(start, &visited);
    }

    fn dfs_recursive(&self, vertex: Agent, visited: &HashMap<String, bool>) {
        if let Some(neighbors) = self.adjacency_list.get(&vertex.identification.name) {
            let mut kids_rating = Vec::new();

            for neighbor in neighbors {
                if !visited[&neighbor.identification.name] {
                    self.dfs_recursive(neighbor.clone(), visited);
                    kids_rating.push(neighbor);
                }
            }

            println!(" kids rating {:?}", kids_rating);
            println!("My Rating {:?} ", vertex);
        }
    }
}
