mod graph_utils;
mod args;
mod drawer;

use crate::args::parse_arguments;
use crate::drawer::Drawer;
use crate::graph_utils::graph::Graph;
use crate::graph_utils::layouts::LayoutAlgorithm;
use crate::graph_utils::layouts::casual_force_layout::ForceLayout;

const DEFAULT_NODE_COLOR: (u8, u8, u8, f64) = (255, 105, 180, 1.0); // розовый
const DEFAULT_EDGE_COLOR: (u8, u8, u8, f64) = (0, 0, 0, 1.0);       // чёрный
const DEFAULT_BACKGROUND_COLOR: (u8, u8, u8, f64) = (245, 245, 220, 1.0); // светло-бежевый

fn main() {
    let res = parse_arguments();
    if res.is_err() {
        println!("{}", res.err().unwrap());
        return;
    }
    let (filepath, width, height, iterations) = res.unwrap();

    let graph = Graph::from_file(filepath.as_str()).unwrap();

    let layout_alg = ForceLayout::new(width, height, graph.size(), iterations);
    let layout = layout_alg.layout(&graph).unwrap();

    // TODO: осталось прикрутить отрисовку
    // print!("{:#?}", layout);

    let drawer = Drawer::new(
        width,
        height,
        DEFAULT_NODE_COLOR,
        DEFAULT_EDGE_COLOR,
        DEFAULT_BACKGROUND_COLOR,
    );
    let path = "results/graph_layout.png";
    let result = drawer.draw_graph(&graph, &layout, path);
    if result.is_err() {
        println!("{}", result.err().unwrap());
    }
}


