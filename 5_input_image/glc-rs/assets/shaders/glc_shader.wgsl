//#import bevy_pbr::mesh_view_bind_group;
struct View {
    view_proj: mat4x4<f32>;
    inverse_view: mat4x4<f32>;
    projection: mat4x4<f32>;
    world_position: vec3<f32>;
    near: f32;
    far: f32;
    width: f32;
    height: f32;
};

[[group(0), binding(0)]]
var<uniform> view: View;


// #import bevy_pbr::mesh_struct;
struct Mesh {
    model: mat4x4<f32>;
    inverse_transpose_model: mat4x4<f32>;
    // 'flags' is a bit field indicating various options. u32 is 32 bits so we have up to 32 options.
    flags: u32;
};

[[group(1), binding(0)]]
var<uniform> mesh: Mesh;


struct Vertex {
    [[location(0)]]
    position: vec3<f32>;
    [[location(1)]]
    normal: vec3<f32>;
    [[location(2)]]
    uv: vec2<f32>;
    [[location(3)]]
    i_pos_scale: vec4<f32>;
    [[location(4)]]
    i_color: vec4<f32>;
};

struct VertexOut {
    [[builtin(position)]]
    clip_position: vec4<f32>;
    [[location(0)]]
    color: vec4<f32>;    
};

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOut {
    let position = vertex.position * vertex.i_pos_scale.w + vertex.i_pos_scale.xyz;
    let world_position = mesh.model * vec4<f32>(position, 1.0);

    var out: VertexOut;
    out.clip_position = view.view_proj * world_position;
    out.color = vertex.i_color;
    return out;
}

[[stage(fragment)]]
fn fragment(in: VertexOut) -> [[location(0)]] vec4<f32> {
    return in.color;
}
