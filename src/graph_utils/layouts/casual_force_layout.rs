use crate::graph_utils::graph::{Graph};
use super::LayoutAlgorithm;
use super::GraphLayout;

use std::collections::HashMap;

const EPS: f32 = 1e-6;

pub struct ForceLayout {
    k: f32,
    //gravity: f64,
    iterations: u32,
    wh_ratio: f32,
}

impl ForceLayout {
    pub fn new(width:u32, height: u32, node_count: usize, iterations: u32) -> ForceLayout {
        let wh_ratio = width as f32 / height as f32;
        let k = (wh_ratio / node_count as f32).sqrt();
        ForceLayout {
            k,
            iterations,
            wh_ratio,
        }
    }
}

impl LayoutAlgorithm for ForceLayout {
    fn layout(self, g: &Graph) -> Result<GraphLayout, String> {
        println!("k {}", self.k);

        let mut layout = GraphLayout::new(g, self.wh_ratio);

        println!("init poses: \n {:#?}", &layout);

        // TODO: Перенести в параметры алгоритма
        let mut t = 0.1_f32;
        let cool = 0.95_f32;

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

                    let mut dist_temp = dx.powf(2.0) + dy.powf(2.0);
                    if dist_temp < EPS {
                        dist_temp = EPS
                    }
                    let distance = dist_temp.sqrt();
                    let repulsion = self.k.powi(2)/distance;

                    let x_displacement = repulsion * dx / distance;
                    let y_displacement = repulsion * dy / distance;

                    if i == 1 && j == 2 {
                        println!("distance - {}", distance);
                        println!("repulsion {}", repulsion);
                    }

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

                let mut dist_temp = dx.powf(2.0) + dy.powf(2.0);
                if dist_temp < EPS {
                    dist_temp = EPS
                }
                let distance = dist_temp.sqrt();
                let attraction = distance.powf(2.0) / self.k;

                if node_i == 1 && node_j == 2 {
                    println!("distance - {}", distance);
                    println!("attraction {}", attraction);
                }

                let x_displacement = attraction * dx / distance;
                let y_displacement = attraction * dy / distance;

                forces.entry(node_i)
                    .and_modify(|f| {
                        f.0 -= x_displacement;
                        f.1 -= y_displacement;
                    });
                forces.entry(node_j)
                    .and_modify(|f| {
                        f.0 += x_displacement;
                        f.1 += y_displacement;
                    });
            }

            for node in &g.nodes {
                let node_id = node.node_id;
                let (fx, fy) = forces[&node_id];
                let displacement = (fx.powf(2.0) + fy.powf(2.0)).sqrt();
                if displacement > 0.0 {
                    let step = displacement.min(t);
                    let dx = fx/ displacement * step;
                    let dy = fy / displacement * step;

                    layout.positions.entry(node_id).and_modify(|(x, y)| {
                        *x += dx;
                        *y += dy;
                    });
                }
            }

            t *= cool;
            if t < 1e-3 { break; }

            println!("{:#?}", &layout);
        }

        Ok(layout)
    }
}