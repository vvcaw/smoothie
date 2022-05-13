use crate::element::Element;
use crate::renderer::{Vertex, WithId};
use lyon::lyon_tessellation::VertexBuffers;
use lyon::math::point;
use lyon::path::Path;
use lyon::tessellation::{
    BuffersBuilder, FillOptions, FillRule, FillTessellator, LineCap, StrokeOptions,
    StrokeTessellator,
};

#[derive(Clone)]
pub struct Arrow {
    x: f64,
    y: f64,
    stroke: bool,
    fill: bool,
    scale: f32,
    angle: f32,
}

impl Arrow {
    pub fn new(scale: f32) -> Self {
        Arrow {
            x: 0.0,
            y: 0.0,
            stroke: true,
            fill: false, // TODO: Since both stroke and fill vertices use the same buffer, the colors of the fill / stroke vertices are the same, to avoid this, maybe add a fill and a stroke color to the Primitive and the vertex shader stage, as this will be needed for any shape that is rendered
            scale,
            angle: 0.0,
        }
    }
}

impl Element for Arrow {
    fn render(&self, geometry: &mut VertexBuffers<Vertex, u16>, primitive_id: usize) {
        // Build a Path for the arrow
        let mut builder = Path::builder();
        builder.begin(point(-1.0, -0.2));
        builder.line_to(point(0.0, -0.2));
        builder.line_to(point(0.0, -0.8));
        builder.line_to(point(1.0, 0.0));
        builder.line_to(point(0.0, 0.8));
        builder.line_to(point(0.0, 0.2));
        builder.line_to(point(-1.0, 0.2));
        builder.close();
        let arrow_path = builder.build();

        // Set custom tolerance
        let tolerance = 0.02;

        if self.fill {
            let mut fill_tess = FillTessellator::new();

            fill_tess
                .tessellate_path(
                    &arrow_path,
                    &FillOptions::tolerance(tolerance).with_fill_rule(FillRule::NonZero),
                    &mut BuffersBuilder::new(geometry, WithId(primitive_id as u32)),
                )
                .unwrap();
        }

        if self.stroke {
            let mut stroke_tess = StrokeTessellator::new();

            stroke_tess
                .tessellate_path(
                    &arrow_path,
                    &StrokeOptions::tolerance(tolerance)
                        .with_line_width(0.05)
                        .with_line_cap(LineCap::Round),
                    &mut BuffersBuilder::new(geometry, WithId(primitive_id as u32)),
                )
                .unwrap();
        }
    }

    fn get_scale(&self) -> f32 {
        self.scale
    }

    fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    fn get_angle(&self) -> f32 {
        self.angle
    }

    fn set_angle(&mut self, angle: f32) {
        self.angle = angle;
    }

    fn box_clone(&self) -> Box<dyn Element + Send> {
        Box::new((*self).clone())
    }
}
