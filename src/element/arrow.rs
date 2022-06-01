use crate::element::Element;
use crate::renderer::{Vertex, WithId};
use crate::{element, Easing, Keyframe};
use lyon::lyon_tessellation::VertexBuffers;
use lyon::math::point;
use lyon::path::Path;
use lyon::tessellation::{
    BuffersBuilder, FillOptions, FillRule, FillTessellator, LineCap, StrokeOptions,
    StrokeTessellator,
};
use std::borrow::{Borrow, BorrowMut};
use std::mem;

#[derive(Clone)]
pub struct Arrow {
    pub x: f32,
    pub y: f32,
    pub stroke: bool,
    pub fill: bool,
    pub scale: f32,
    pub angle: f32,
    pub(crate) keyframes: Vec<Keyframe<Arrow>>,
    pub(crate) id: usize,
}

impl Arrow {
    /// Add keyframes to the given **Element**, this method is automatically invoked by the `animate!` macro!
    pub fn add_keyframe(
        &mut self,
        keyframe_data: (fn(&mut Arrow, f32), f32, f32, f32, f32, Easing),
    ) {
        let (setter_fn, start_value, end_value, start_time, duration, easing) = keyframe_data;

        self.keyframes.push(Keyframe {
            setter_fn,
            start_value,
            end_value,
            start_time,
            duration,
            easing,
        });
    }
}

impl crate::element::private::Element for Arrow {
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

    fn box_clone(&self) -> Box<dyn crate::element::Element + Send> {
        Box::new((*self).clone())
    }

    fn update_data_with_keyframes(&mut self, time_since_start: f32) {
        // Evaluate which keyframes need to be updated and update them
        let keyframes = mem::take(&mut self.keyframes);

        keyframes
            .iter()
            .filter(|keyframe| keyframe.is_active(time_since_start))
            .for_each(|keyframe| {
                keyframe.update_keyframe_data(self.borrow_mut(), time_since_start)
            });

        self.keyframes = keyframes;
    }

    fn get_scale(&self) -> f32 {
        self.scale
    }

    fn get_angle(&self) -> f32 {
        self.angle
    }
}

impl Element for Arrow {
    fn get_id(&self) -> usize {
        self.id
    }
}
