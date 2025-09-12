use crate::graph_utils::graph::{Graph};
use super::LayoutAlgorithm;
use super::GraphLayout;

use std::collections::HashMap;

pub struct ForceLayout {
    k: f32,
    //gravity: f64,
    iterations: u32,
}

impl ForceLayout {
    pub fn new(width:u32, height: u32, node_count: usize, iterations: u32) -> ForceLayout {
        let area = (width * height) as f32;
        let k = (area / node_count as f32).sqrt();
        ForceLayout {
            k,
            iterations,
        }
    }
}

impl LayoutAlgorithm for ForceLayout {
    fn layout(self, g: &Graph) -> GraphLayout{
        let mut layout = GraphLayout::new(g);
        let mut forces:HashMap<u32, (f32, f32)> = HashMap::new();
        for _ in 0..self.iterations {

            for node in &g.nodes {
                forces.insert(node.node_id, (0.0, 0.0));
            }

            for i in 0..g.nodes.len() {
                let node_i = &g.nodes[i].node_id;

                let x_i = layout.positions[node_i].0;
                let y_i = layout.positions[node_i].1;

                for j in i+1..g.nodes.len() {
                    let node_j = &g.nodes[j].node_id;

                    let x_j = layout.positions[node_j].0;
                    let y_j = layout.positions[node_j].1;

                    let dx = x_i - x_j;
                    let dy = y_i - y_j;

                    let distance = (dx.powf(2.0) + dy.powf(2.0)).sqrt();
                    let repulsion = self.k.powi(2)/distance;

                    let x_displacement = repulsion * dx / distance;
                    let y_displacement = repulsion * dy / distance;

                    forces.entry(*node_i)
                        .and_modify(|f| {
                            f.0 += x_displacement;
                            f.1 += y_displacement;
                        });
                    forces.entry(*node_j)
                        .and_modify(|f| {
                            f.0 -= x_displacement;
                            f.1 -= y_displacement;
                        });
                }
            }

            for edge in &g.edges {
                let node_i = edge.source_id;
                let node_j = edge.target_id;

                let x_i = layout.positions[&node_i].0;
                let y_i = layout.positions[&node_i].1;
                let x_j = layout.positions[&node_j].0;
                let y_j = layout.positions[&node_j].1;

                let dx = x_i - x_j;
                let dy = y_i - y_j;

                let distance = (dx.powf(2.0) + dy.powf(2.0)).sqrt();
                let attraction = distance.powf(2.0) / self.k;

                let x_displacement = attraction * dx / distance;
                let y_displacement = attraction * dy / distance;

                forces.entry(node_i)
                    .and_modify(|f| {
                        f.0 += x_displacement;
                        f.1 += y_displacement;
                    });
                forces.entry(node_j)
                    .and_modify(|f| {
                        f.0 -= x_displacement;
                        f.1 -= y_displacement;
                    });
            }
        }

        for node in &g.nodes {
            let node_id = node.node_id;
            layout.positions.entry(node_id).and_modify(|(x, y)| {
                *x += forces[&node_id].0;
                *y += forces[&node_id].1;
            });
        }
        layout
    }
}