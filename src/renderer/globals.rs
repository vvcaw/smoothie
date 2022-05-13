#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
/// A struct to represent global scene / camera - like attributes for the renderer
pub struct Globals {
    pub resolution: [f32; 2],
    pub offset: [f32; 2],
    pub zoom: f32,
    pub _pad: i32,
}
