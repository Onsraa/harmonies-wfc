use crate::components::grid::hex::display::polygon_corners;
use crate::components::grid::hex::display::{hex_to_pixel, HexInstances};
use crate::components::grid::hex::Hex;
use crate::resources::grid::GridLayout;
use bevy::asset::Assets;
use bevy::color::{Color, Srgba};
use bevy::math::Vec3;
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{Commands, Mesh, Mesh3d, Res, ResMut, Transform};
use bevy::render::mesh::{Indices, PrimitiveTopology};

pub fn setup_grid(
    mut commands: Commands,
    grid_layout: Res<GridLayout>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut transforms = Vec::new();
    let mut colors = Vec::new();

    for x in 0..grid_layout.grid_size.x as i32 {
        for y in 0..grid_layout.grid_size.y as i32 {
            let hex = Hex::from_axial(x, y);
            let pixel_pos = hex_to_pixel(&grid_layout, hex);

            commands.spawn(hex);

            transforms.push(Transform::from_translation(pixel_pos.extend(0.0)));
            colors.push(Color::srgb(0.0, 1.0, 0.0));
        }
    }

    commands.spawn(HexInstances { transforms, colors });

    commands.spawn((
        Mesh3d(meshes.add(create_hex_mesh_from_layout(&grid_layout))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Srgba::hex("#ffd891").unwrap().into(),
            unlit: true,
            ..Default::default()
        })),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    ));
}

pub fn create_hex_mesh_from_layout(layout: &GridLayout) -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
    );

    let unit_hex = Hex::new(0, 0, 0);
    let corners = polygon_corners(layout, unit_hex);

    let mut positions = Vec::with_capacity(7);
    let mut normals = Vec::with_capacity(7);
    let mut uvs = Vec::with_capacity(7);

    positions.push([0.0, 0.0, 0.0]);
    normals.push([0.0, 1.0, 0.0]);
    uvs.push([0.5, 0.5]);

    for corner in corners.iter() {
        positions.push([corner.x, 0.0, corner.y]);
        normals.push([0.0, 1.0, 0.0]);

        let normalized_x = (corner.x / layout.hex_size + 1.0) * 0.5;
        let normalized_y = (corner.y / layout.hex_size + 1.0) * 0.5;
        uvs.push([normalized_x, normalized_y]);
    }

    let mut indices = Vec::with_capacity(18);
    for i in 0..6 {
        let next = (i + 1) % 6;
        indices.extend_from_slice(&[0, next + 1, i + 1]);
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}
