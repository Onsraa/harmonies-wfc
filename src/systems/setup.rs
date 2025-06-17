use crate::components::cell::Cell;
use crate::components::grid::Grid;
use bevy::color::palettes::css::SILVER;
use bevy::prelude::*;


pub fn initialize(
    mut commands: Commands,
    grid: Option<Res<Grid>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    if let Some(grid) = &grid {
        for x in 0..grid.width {
            for y in 0..grid.height {
                commands.spawn(Cell::new(x, y));
            }
        }
    }

    // test de Spawn pour vérifier que les cellules sont bien créées
    // Load and spawn your GLTF model
    //let rock_handle: Handle<Scene> = asset_server.load("tiles/tile_rock.gltf");
    // let rock_handle = asset_server.load::<Scene>("tiles/tile_rock.gltf#Scene0");
    // commands.spawn((
    //     SceneRoot(rock_handle),
    //     Transform::from_xyz(0.0, 0.0, 0.0)
    //         .with_scale(Vec3::splat(1.0)),
    // ));

    let leave_anim = asset_server.load::<Scene>("tiles/leaves_animation.gltf#Scene0");
    commands.spawn((
        SceneRoot(leave_anim),
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_scale(Vec3::splat(1.0)),
    ));

    
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

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
    ));
}
