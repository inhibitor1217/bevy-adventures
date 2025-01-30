use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

const MUSGRAVE_SHADER: &str = include_str!("../res/shaders/musgrave_fbm3d_mat.wgsl");

pub const MUSGRAVE_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(18507865731074489240147604704102871);

pub struct MusgravePlugin;

impl Plugin for MusgravePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<MusgraveMaterial>::default());
        app.add_systems(Update, update_uniforms);

        let mut shaders = app.world_mut().resource_mut::<Assets<Shader>>();
        let musgrave_shader = Shader::from_wgsl(MUSGRAVE_SHADER, file!());
        shaders.insert(&MUSGRAVE_SHADER_HANDLE, musgrave_shader);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MusgraveConfig {
    pub offset: Vec3,
    pub scale: f32,
    pub detail: u32,
    pub dimension: f32,
    pub lacunarity: f32,
}

impl Default for MusgraveConfig {
    fn default() -> Self {
        Self {
            offset: Vec3::ZERO,
            scale: 1.0,
            detail: 5,
            dimension: 2.0,
            lacunarity: 2.0,
        }
    }
}

#[derive(Debug, Clone, AsBindGroup, Asset, TypePath, Default)]
pub struct MusgraveMaterial {
    #[uniform(0)]
    offset: Vec4,

    #[uniform(1)]
    config_uniform: Vec4,

    pub config: MusgraveConfig,
}

impl MusgraveMaterial {
    pub fn update_uniforms(&mut self) {
        self.offset = Vec4::new(
            self.config.offset.x,
            self.config.offset.y,
            self.config.offset.z,
            0.0,
        );
        self.config_uniform = Vec4::new(
            self.config.scale,
            self.config.detail as f32,
            self.config.dimension,
            self.config.lacunarity,
        );
    }
}

impl Material for MusgraveMaterial {
    fn fragment_shader() -> ShaderRef {
        MUSGRAVE_SHADER_HANDLE.into()
    }
}

fn update_uniforms(mut materials: ResMut<Assets<MusgraveMaterial>>) {
    for mat in materials.iter_mut() {
        mat.1.update_uniforms();
    }
}
