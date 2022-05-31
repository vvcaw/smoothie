mod arrow;

pub use arrow::Arrow;

pub trait Element: private::Element {
    /// Get **id**
    fn get_id(&self) -> usize;
}

// A bit of cheating to implement a partially private trait that is not exposed as API
pub(crate) mod private {
    use crate::animation::Keyframe;
    use crate::renderer::Vertex;
    use lyon::tessellation::VertexBuffers;

    pub trait Element {
        /// Tessellates the given **Element** and adds it to the geometry buffer
        fn render(&self, geometry: &mut VertexBuffers<Vertex, u16>, primitive_id: usize);

        /// Clones inside a **Box**
        fn box_clone(&self) -> Box<dyn crate::element::Element + Send>;

        /// Update **keyframe** data
        fn update_data_with_keyframes(&mut self, time_since_start: f32);

        /// Get **scale**
        fn get_scale(&self) -> f32;

        /// Get **angle**
        fn get_angle(&self) -> f32;
    }
}

// Implement the `Clone` trait for Box<dyn Element + Send>
impl Clone for Box<dyn Element + Send> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}
