#import bevy_pbr::forward_io::VertexOutput
#import "shaders/simplex_noise.wgsl"::noise2d

@group(1) @binding(0)
var<uniform> time: f32;

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let noise_value = noise2d(uv * 8.0);
    let color = (noise_value + 1.0) * 0.5;
    return vec4<f32>(color, color, color, 1.0);
}
