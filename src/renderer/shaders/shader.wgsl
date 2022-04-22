// Vertex shader

struct Primitive {
    color: vec4<f32>;
    translate: vec2<f32>;
    z_index: i32;
    width: f32;
    angle: f32;
    scale: f32;
    pad1: i32;
    pad2: i32;
};

// Primitive struct recieved from CPU in [[stage(vertex)]]
struct Primitives {
    // TODO: Find out if this is necessary [[stride(48)]]
    primitives: array<Primitive, 256>;
};

// Bind group with index 0 and binding with index 0
[[group(0), binding(0)]] var<uniform> u_primitives: Primitives;

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec4<f32>;
};

[[stage(vertex)]]
fn vs_main(
    [[location(0)]] a_position: vec3<f32>,
    [[location(1)]] a_normal: vec3<f32>,
    [[location(2)]] a_prim_id: u32
) -> VertexOutput {
    // Get current primitive data from uniform buffer
    var prim: Primitive = u_primitives.primitives[a_prim_id];

    var out: VertexOutput;

    out.color = prim.color;
    out.clip_position = vec4<f32>(a_position, 1.0);
    return out;
}

// Fragment shader

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return in.color;
}