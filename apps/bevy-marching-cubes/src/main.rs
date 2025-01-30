use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

fn main() {
    let mut app = App::new();

    let mut default_plugins_builder = DefaultPlugins.build();

    #[cfg(target_arch = "wasm32")]
    {
        default_plugins_builder = default_plugins_builder.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        });
    }

    #[cfg(debug_assertions)]
    {
        default_plugins_builder = default_plugins_builder.set(AssetPlugin {
            watch_for_changes_override: Some(true),
            ..default()
        });
    }

    app.add_plugins(default_plugins_builder);

    app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin);
    app.add_plugins(bevy_diagnostics_ui::DiagnosticsUiPlugin);
    app.add_systems(Startup, setup);

    app.add_plugins(MaterialPlugin::<SimplexNoiseMaterial>::default());
    app.add_systems(Update, simplex_noise_update_uniforms);

    app.add_plugins(MaterialPlugin::<MusgraveMaterial>::default());
    app.add_systems(Update, musgrave_update_uniforms);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MusgraveMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(1.0, 1.0))),
        MeshMaterial3d(materials.add(MusgraveMaterial::default())),
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
}

#[derive(Debug, Clone, AsBindGroup, Asset, TypePath, Default)]
struct SimplexNoiseMaterial {
    #[uniform(0)]
    time: Vec4,
}

impl Material for SimplexNoiseMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/simplex_noise_mat.wgsl".into()
    }
}

fn simplex_noise_update_uniforms(
    mut materials: ResMut<Assets<SimplexNoiseMaterial>>,
    time: Res<Time>,
) {
    for mat in materials.iter_mut() {
        mat.1.time = Vec4::new(time.elapsed_secs(), time.delta_secs(), 0.0, 0.0);
    }
}

#[derive(Debug, Clone, Copy)]
struct MusgraveConfig {
    offset: Vec3,
    scale: f32,
    detail: u32,
    dimension: f32,
    lacunarity: f32,
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

impl From<MusgraveConfig> for Vec4 {
    fn from(config: MusgraveConfig) -> Self {
        Vec4::new(
            config.scale,
            config.detail as f32,
            config.dimension,
            config.lacunarity,
        )
    }
}

#[derive(Debug, Clone, AsBindGroup, Asset, TypePath, Default)]
struct MusgraveMaterial {
    #[uniform(0)]
    offset: Vec3,

    #[uniform(1)]
    config_uniform: Vec4,

    config: MusgraveConfig,
}

impl MusgraveMaterial {
    fn update_uniforms(&mut self) {
        self.offset = self.config.offset;
        self.config_uniform = self.config.into();
    }
}

impl Material for MusgraveMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/musgrave_fbm3d_mat.wgsl".into()
    }
}

fn musgrave_update_uniforms(mut materials: ResMut<Assets<MusgraveMaterial>>) {
    for mat in materials.iter_mut() {
        mat.1.update_uniforms();
    }
}
