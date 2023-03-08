// Vertex shader

struct CameraUniform {
    view_proj: mat4x4<f32>,
    position: vec3<f32>,
    padding: f32,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct SkyUniform {
    sun_angle: vec3<f32>,
    padding: f32,
}
@group(1) @binding(0)
var<uniform> sky: SkyUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) element_type: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>, 
    @location(0) base_position: vec3<f32>,
    @location(1) element_type: u32,
};

@vertex
fn vs_main(
    in: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(in.position + camera.position, 1.0);
    out.base_position = in.position;
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let pos = normalize(in.base_position);
    var color = vec3<f32>(0.0);
    
    let height = (pow(((pos.y + 1.0) / 2.0) - 0.5, 0.2) / 1.75) + 0.5;
    color = mix(vec3<f32>(0.718, 0.816, 0.973), vec3<f32>(0.494, 0.671, 0.996), height);

    let sun_dist = acos(dot(pos, sky.sun_angle));

    color += vec3<f32>(1.0) * max(1.0 - (sun_dist * 10.0), 0.0);

    //let sun =
    //    (1.0 / (abs(pos.x - sky.sun_angle.x) * 100.0)) +
    //    (1.0 / (abs(pos.y - sky.sun_angle.y) * 100.0)) +
    //    (1.0 / (abs(pos.z - sky.sun_angle.z) * 100.0));
    //color += max(vec3<f32>(sun), vec3<f32>(0.0));

    return vec4<f32>(color, 1.0);
}