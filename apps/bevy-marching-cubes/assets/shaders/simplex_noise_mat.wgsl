#import bevy_pbr::forward_io::VertexOutput;

@group(1) @binding(0)
var<uniform> time: f32;

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    let uv = in.uv;
    return vec4<f32>(uv.x, uv.y, 1.0, 1.0);
}
