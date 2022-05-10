mod arrow;

use crate::renderer::Vertex;
use lyon::tessellation::VertexBuffers;

pub use arrow::Arrow;

/// A enum describing all available **Element** types
pub enum ElementType {
    Arrow,
}

/// A trait, that all **Elements** have to implement
pub trait Element {
    /// Tessellate the given **Element** and adds it to the geometry buffer
    fn render(&self, geometry: &mut VertexBuffers<Vertex, u16>, primitive_id: usize);

    /// Returns the scale of the given **Element**
    fn scale(&self) -> f32;

    /// Clones inside a **Box**
    fn box_clone(&self) -> Box<dyn Element + Send>;
}

// Implement the clone trait for Box<dyn Element>
impl Clone for Box<dyn Element + Send> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}
