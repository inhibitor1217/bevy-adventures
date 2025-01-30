#import "shaders/simplex_noise.wgsl"::{noise2d, noise3d}

struct FbmConfig {
  scale: f32,
  detail: f32,
  dimension: f32,
  lacunarity: f32,
};

fn fbm2d(
  pos: vec2<f32>,
  config: FbmConfig,
) -> f32 {
  var value = 0.0;
  var amplitude = 1.0;
  var frequency = config.scale;
  var total_amplitude = 0.0;

  let octaves = i32(config.detail);
  let dimension_inv = 1.0 / config.dimension;

  for (var i = 0; i < octaves; i += 1) {
    value += amplitude * noise2d(pos * frequency);
    total_amplitude += amplitude;

    amplitude *= dimension_inv;
    frequency *= config.lacunarity;
  }

  return value / total_amplitude;
}

fn fbm3d(
  pos: vec3<f32>,
  config: FbmConfig,
) -> f32 {
  var value = 0.0;
  var amplitude = 1.0;
  var frequency = config.scale;
  var total_amplitude = 0.0;

  let octaves = i32(config.detail);
  let dimension_inv = 1.0 / config.dimension;
  
  for (var i = 0; i < octaves; i += 1) {
    value += amplitude * noise3d(pos * frequency);
    total_amplitude += amplitude;

    amplitude *= dimension_inv;
    frequency *= config.lacunarity;
  }

  return value / total_amplitude;
}
