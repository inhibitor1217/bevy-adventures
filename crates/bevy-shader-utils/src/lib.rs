mod musgrave;
pub use musgrave::{MusgraveConfig, MusgraveMaterial};

use bevy::prelude::*;

const SIMPLEX_NOISE_SHADER: &str = include_str!("../res/shaders/simplex_noise.wgsl");
const FBM_SHADER: &str = include_str!("../res/shaders/fbm.wgsl");

pub const SIMPLEX_NOISE_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(38533812020171569453094116057695321);

pub const FBM_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(106950878680214389185493850177621179);

pub struct ShaderUtilsPlugin;

impl Plugin for ShaderUtilsPlugin {
    fn build(&self, app: &mut App) {
        let mut shaders = app.world_mut().resource_mut::<Assets<Shader>>();

        let mut simplex_noise_shader = Shader::from_wgsl(SIMPLEX_NOISE_SHADER, file!());
        simplex_noise_shader.set_import_path("bevy_shader_utils::simplex_noise");
        shaders.insert(&SIMPLEX_NOISE_SHADER_HANDLE, simplex_noise_shader);

        let mut fbm_shader = Shader::from_wgsl(FBM_SHADER, file!());
        fbm_shader.set_import_path("bevy_shader_utils::fbm");
        shaders.insert(&FBM_SHADER_HANDLE, fbm_shader);

        app.add_plugins(musgrave::MusgravePlugin);
    }
}
