// Vertex shader

struct Globals {
    resolution: vec2<f32>;
    offset: vec2<f32>;
    zoom: f32;
    _pad: i32;
};

struct Primitive {
    color: vec4<f32>;
    translate: vec2<f32>;
    z_index: i32;
    angle: f32;
    scale: f32;
    pad1: i32;
    pad2: i32;
    pad3: i32;
};

// Primitive struct recieved from CPU in [[stage(vertex)]]
struct Primitives {
    // TODO: Find out if this is necessary [[stride(48)]]
    primitives: array<Primitive, 256>;
};

// Bind group with index 0 and binding with index 0
[[group(0), binding(0)]] var<uniform> u_primitives: Primitives;

// Bind group with index 0 and binding with index 1
[[group(0), binding(1)]] var<uniform> u_globals: Globals;

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec4<f32>;
};

[[stage(vertex)]]
fn vs_main(
    [[location(0)]] a_position: vec2<f32>,
    [[location(1)]] a_normal: vec2<f32>,
    [[location(2)]] a_prim_id: u32
) -> VertexOutput {
    // Get current primitive data from uniform buffer
    var prim: Primitive = u_primitives.primitives[a_prim_id];

    var res = u_globals.resolution;

    var out: VertexOutput;

    var rotation = mat2x2<f32>(
       vec2<f32>(cos(prim.angle), -sin(prim.angle)),
       vec2<f32>(sin(prim.angle), cos(prim.angle))
    );

    var local_pos = (a_position * prim.scale) * rotation;
    var world_pos = local_pos - u_globals.offset + prim.translate;

    var transformed_pos = world_pos * u_globals.zoom / (res / length(res));

    out.color = prim.color;
    out.clip_position = vec4<f32>(transformed_pos, f32(prim.z_index), 1.0);
    return out;
}

// Fragment shader

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return in.color;
}