#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
// Because the struct implements the `bytemuck::Pod` trait, it may not contain any padding bytes
/// A **primitive** that is passed to enable uniform rendering for all **Elements**.
pub struct Primitive {
    pub(crate) color: [f32; 4],     // 16
    pub(crate) z_index: i32,        // 4
    pub(crate) width: f32,          // 4
    pub(crate) angle: f32,          // 4
    pub(crate) scale: f32,          // 4
    pub(crate) translate: [f32; 3], // 3!
    pub(crate) _pad1: i32, // 1! -> Padding for making sure that we end without padding bytes
}

impl Primitive {
    pub(crate) const DEFAULT: Self = Primitive {
        color: [0.0; 4],
        z_index: 0,
        width: 0.0,
        angle: 0.0,
        scale: 1.0,
        translate: [0.0; 3],
        _pad1: 0,
    };
}
