#import bevy_pbr::forward_io::VertexOutput
#import "shaders/simplex_noise.wgsl"::noise3d

@group(2) @binding(0)
var<uniform> time: vec4<f32>;

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let noise_value = noise3d(vec3(uv * 8.0, time.x * 0.5));
    let power = (noise_value + 1.0) * 0.5;
    return vec4<f32>(power, power, power, 1.0);
}
