use crate::graph_utils::graph::Graph;
use crate::graph_utils::layouts::GraphLayout;
use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;

fn tuple_to_rgb_color(tuple: (u8, u8, u8, f64)) -> RGBAColor {
    RGBAColor(tuple.0, tuple.1, tuple.2, tuple.3)
}

pub struct Drawer {
    width: u32,
    height: u32,
    node_color: (u8, u8, u8, f64),
    edge_color: (u8, u8, u8, f64),
    background_color: (u8, u8, u8, f64),
}

impl Drawer {
    pub fn new(
        width: u32,
        height: u32,
        node_color: (u8, u8, u8, f64),
        edge_color: (u8, u8, u8, f64),
        background_color: (u8, u8, u8, f64),
    ) -> Self {
        Self {
            width,
            height,
            node_color,
            edge_color,
            background_color,
        }
    }

    // TODO: надо координаты нормализовать. В плане, чтобы граф не выходил за пределы холста. А в
    // идеале еще и центрировать его
    pub fn draw_graph(
        &self,
        graph: &Graph,
        layout: &GraphLayout,
        path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new(path, (self.width, self.height)).into_drawing_area();

        let background_color = tuple_to_rgb_color(self.background_color);
        let node_color = tuple_to_rgb_color(self.node_color);
        let edge_color = tuple_to_rgb_color(self.edge_color);

        let edge_style = ShapeStyle {
            color: edge_color,
            filled: false,
            stroke_width: 1,
        };

        let root = root.apply_coord_spec(Cartesian2d::<RangedCoordf32, RangedCoordf32>::new(
            0f32..1f32,
            0f32..1f32,
            (0..(self.width as i32), 0..(self.height as i32)),
        ));

        root.fill(&background_color)?;

        let node_and_label = |id: u32, x: f32, y: f32| {
            return EmptyElement::at((x, y))
                + Circle::new((0, 0), 3, ShapeStyle::from(&node_color).filled())
                + Text::new(format!("{}", id), (10, 0), ("sans-serif", 15.0).into_font());
        };

        for node in &graph.nodes {
            let (x, y) = layout.positions[&node.node_id];
            root.draw(&node_and_label(node.node_id, x, y))?;
        }

        for edge in &graph.edges {
            let (x_source, y_source) = layout.positions[&edge.source_id];
            let (x_dest, y_dest) = layout.positions[&edge.target_id];
            root.draw(&PathElement::new(
                vec![(x_source, y_source), (x_dest, y_dest)],
                edge_style.clone(),
            ))?;
        }

        root.present()?;
        
        Ok(())
    }
}
