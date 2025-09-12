use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct Node<T = ()> {
    pub node_id: u32,
    pub data: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Edge<T = ()> {
    pub edge_id: u32,
    pub source_id: u32,
    pub target_id: u32,
    pub data: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Graph {
    pub(super) nodes: Vec<Node>,
    pub(super) edges: Vec<Edge>,
}

impl Graph {
    pub fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Graph {
        Graph {
            nodes,
            edges
        }
    }

    pub fn from_file(filepath: &str) -> Result<Graph, String> {
        let mut file = File::open(filepath).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let g: Graph = serde_json::from_str(&contents).unwrap();
        Ok(g)
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }
}