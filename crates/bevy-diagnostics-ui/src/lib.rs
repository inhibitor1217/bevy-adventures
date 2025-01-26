use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

/// Marker component for the diagnostics UI root node
#[derive(Component)]
struct DiagnosticsUiRoot;

/// Marker component for the FPS text
#[derive(Component)]
struct FpsText;

/// Bevy plugin providing diagnostics UI feature.
#[derive(Default)]
pub struct DiagnosticsUiPlugin;

impl Plugin for DiagnosticsUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_diagnostics_ui);
        app.add_systems(Update, update_diagnostics_fps_text);
    }
}

fn setup_diagnostics_ui(mut commands: Commands) {
    // UI root node
    commands
        .spawn((
            DiagnosticsUiRoot,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(4.0),
                right: Val::Px(4.0),
                padding: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            BackgroundColor(Color::BLACK.with_alpha(0.5)),
            GlobalZIndex(i32::MAX),
        ))
        .with_child((
            FpsText,
            Text::new("FPS: N/A"),
            TextFont {
                font_size: 12.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
}

fn update_diagnostics_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            text.0 = format!("FPS: {:.2}", fps);
        } else {
            text.0 = "FPS: N/A".to_string();
        }
    }
}
