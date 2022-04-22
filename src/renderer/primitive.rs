#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
/// A **primitive** that is passed to enable uniform rendering for all **Elements**.
pub struct Primitive {
    pub(crate) color: [f32; 4],
    pub(crate) translate: [f32; 2],
    pub(crate) z_index: i32,
    pub(crate) width: f32,
    pub(crate) angle: f32,
    pub(crate) scale: f32,
    pub(crate) _pad1: i32,
    pub(crate) _pad2: i32,
}

impl Primitive {
    pub(crate) const DEFAULT: Self = Primitive {
        color: [0.0; 4],
        translate: [0.0; 2],
        z_index: 0,
        width: 0.0,
        angle: 0.0,
        scale: 1.0,
        _pad1: 0,
        _pad2: 0,
    };
}
