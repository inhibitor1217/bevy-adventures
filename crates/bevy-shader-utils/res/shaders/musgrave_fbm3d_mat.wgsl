#import bevy_pbr::forward_io::VertexOutput
#import bevy_shader_utils::fbm::{FbmConfig, fbm2d}

@group(2) @binding(0)
var<uniform> offset: vec4<f32>;

@group(2) @binding(1)
var<uniform> config: FbmConfig;

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let pos = uv + offset.xy;
    let value = (fbm2d(pos, config) + 1.0) * 0.5;
    return vec4<f32>(value, value, value, 1.0);
}
