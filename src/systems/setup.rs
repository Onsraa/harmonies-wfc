use bevy::color::palettes::css::SILVER;
use bevy::prelude::*;

pub fn initialize(
    mut commands: Commands,
) {

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));
}
