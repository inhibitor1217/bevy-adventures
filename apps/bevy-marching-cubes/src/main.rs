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

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SimplexNoiseMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(1.0, 1.0))),
        MeshMaterial3d(materials.add(SimplexNoiseMaterial { time: 0.0 })),
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
}

#[derive(Debug, Clone, AsBindGroup, Asset, TypePath)]
struct SimplexNoiseMaterial {
    #[uniform(0)]
    time: f32,
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
        mat.1.time = time.elapsed_secs();
    }
}
