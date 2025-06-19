use bevy::prelude::*;

pub fn initialize(mut commands: Commands, asset_server: Res<AssetServer>) {
    // test de Spawn pour vérifier que les cellules sont bien créées
    // Load and spawn your GLTF model
    let rock_handle = asset_server.load::<Scene>("tile/tile_rock.gltf#Scene0");
    commands.spawn((
        SceneRoot(rock_handle),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(1.0)),
    ));

    let leave_anim = asset_server.load::<Scene>("tile/leaves_animation.gltf#Scene0");
    commands.spawn((
        SceneRoot(leave_anim),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(1.0)),
    ));

    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 2000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
