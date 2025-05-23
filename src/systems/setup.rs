use crate::components::cell::Cell;
use crate::components::grid::Grid;
use bevy::color::palettes::css::SILVER;
use bevy::prelude::*;

pub fn initialize(
    mut commands: Commands,
    grid: Option<Res<Grid>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Some(grid) = &grid {
        for x in 0..grid.width {
            for y in 0..grid.height {
                commands.spawn(Cell::new(x, y));
            }
        }
    }

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
