use crate::renderer::vertex::Vertex;

use lyon::tessellation::{
    FillVertex, FillVertexConstructor, StrokeVertex, StrokeVertexConstructor,
};

/// This vertex constructor forwards the positions and normals provided by the
/// tessellators and adds a **custom primitive id**.
pub struct WithId(pub u32);

impl FillVertexConstructor<Vertex> for WithId {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        Vertex {
            position: [vertex.position().x, vertex.position().y, 0.0],
            normal: [0.0, 1.0, 0.0],
            prim_id: self.0,
        }
    }
}

impl StrokeVertexConstructor<Vertex> for WithId {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> Vertex {
        Vertex {
            position: [
                /*vertex.position_on_path().x,
                vertex.position_on_path().y,*/
                vertex.position().x,
                vertex.position().y,
                0.0,
            ], // TODO: Find out what this is
            normal: [0.0, 1.0, 0.0],
            prim_id: self.0,
        }
    }
}
