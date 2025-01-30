use bevy::prelude::*;

use bevy_shader_utils::{MusgraveMaterial, ShaderUtilsPlugin};

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

    app.add_plugins(ShaderUtilsPlugin);

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
