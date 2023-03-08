// Vertex shader

struct CameraUniform {
    view_proj: mat4x4<f32>,
    position: vec3<f32>,
    padding: f32,
};
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct SkyUniform {
    sun_angle: vec3<f32>,
    padding: f32,
}
@group(2) @binding(0)
var<uniform> sky: SkyUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) index: u32, 
    @location(3) data: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(2) index: u32,
    @location(3) light: vec4<f32>,
    @location(4) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(
    in: VertexInput,
) -> VertexOutput {
    let data = decode(in.data);

    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(in.position, 1.0);
    out.normal = in.normal;
    out.index = in.index;
    out.light = data.light;
    out.tex_coords = data.tex_coords;
    return out;
}

// Fragment shader

@group(0) @binding(0)
var texture_array: binding_array<texture_2d<f32>>;
@group(0) @binding(1)
var sampler_: sampler;

struct Data {
    light: vec4<f32>,
    tex_coords: vec2<f32>,
}

fn decode(i: u32) -> Data {
    return Data(
        vec4<f32>(
            f32(i >> 28u) / 15.0,
            f32((i << 4u) >> 28u) / 15.0,
            f32((i << 8u) >> 28u) / 15.0,
            f32((i << 12u) >> 28u) / 15.0,
        ),
        vec2<f32>(
            f32((i << 16u) >> 31u),
            f32((i << 17u) >> 31u),
        )
    );
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let diffuse: vec4<f32> = textureSample(
        texture_array[in.index],
        sampler_,
        in.tex_coords,
    );

    let directional_light = max(dot(sky.sun_angle, in.normal), 0.0) * in.light.w;
    let ambient_light = vec3<f32>(0.1, 0.1, 0.1);
    
    let light = max(vec3<f32>(
        in.light.x + directional_light,
        in.light.y + directional_light,
        in.light.z + directional_light,
    ), ambient_light);
        
    return vec4<f32>(diffuse.xyz * min(light, vec3<f32>(1.0)), diffuse.w);
}