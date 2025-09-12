use std::collections::HashMap;
use rand::Rng;
use crate::graph_utils::graph::Graph;

pub mod casual_force_layout;

#[derive(Debug)]
pub struct GraphLayout {
    pub positions: HashMap<u32, (f32, f32)>
}

impl GraphLayout {
    pub fn new(g: &Graph) -> GraphLayout {
        let mut positions = HashMap::new();
        let mut rng = rand::rng();

        for node in &g.nodes {
            let x = rng.random_range(0.0..1.0);
            let y = rng.random_range(0.0..1.0);
            positions.insert(node.node_id, (x, y));
        }
        GraphLayout {
            positions
        }
    }
}

pub trait LayoutAlgorithm {
    fn layout(self, g: &Graph) -> GraphLayout;
}