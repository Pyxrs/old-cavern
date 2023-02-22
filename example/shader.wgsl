// Vertex shader

struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

@group(2) @binding(0)
var<uniform> sun_angle: vec2<f32>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) index: u32,
    @location(3) light: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) index: u32,
    @location(2) light: u32,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    out.index = model.index;
    out.light = model.light;
    return out;
}

// Fragment shader

@group(0) @binding(0)
var texture_array: binding_array<texture_2d<f32>>;
@group(0) @binding(1)
var sampler_: sampler;

struct Light {
    r: f32,
    g: f32,
    b: f32,
    szp: f32,
    szn: f32,
    sxp: f32,
    sxn: f32,
}

fn decode_light(i: u32) -> Light {
    return Light(
        f32(i >> 28u) / 15.0,
        f32((i << 4u) >> 28u) / 15.0,
        f32((i << 8u) >> 28u) / 15.0,
        f32((i << 12u) >> 28u) / 15.0,
        f32((i << 16u) >> 28u) / 15.0,
        f32((i << 20u) >> 28u) / 15.0,
        f32((i << 24u) >> 28u) / 15.0,
    );
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let diffuse: vec4<f32> = textureSample(
        texture_array[in.index],
        sampler_,
        in.tex_coords,
    );

    let light = decode_light(in.light);
    let sunlight =
        (light.szp * ((sun_angle.y + 1.0) / 2.0)) +
        (light.szn * ((sun_angle.y - 1.0) / 2.0)) +
        (light.sxp * ((sun_angle.x + 1.0) / 2.0)) +
        (light.sxn * ((sun_angle.x - 1.0) / 2.0));
        
    return diffuse * vec4<f32>(light.r + sunlight, light.g + sunlight, light.b + sunlight, 1.0);
}