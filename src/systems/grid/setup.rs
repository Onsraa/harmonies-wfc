use crate::components::grid::hex::display::{hex_to_pixel, polygon_corners};
use crate::components::grid::hex::Hex;
use crate::resources::grid::GridLayout;
use bevy::asset::Handle;
use bevy::ecs::system::SystemState;
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::*;

pub fn setup_grid(
    world: &mut World,
    params: &mut SystemState<(
        Res<GridLayout>,
        ResMut<Assets<Mesh>>,
        ResMut<Assets<StandardMaterial>>,
    )>,
) {
    let (grid_layout, mut meshes, mut materials) = params.get_mut(world);
    let transparent_material = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 1.0, 1.0, 0.2).into(),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });
    let base_material = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 1.0, 1.0, 0.5).into(),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });
    let hexagon = Extrusion::new(create_hex_polygon_from_layout(&grid_layout), 0.0);
    let mut hexes: Vec<_> = vec![];
    for q in (-grid_layout.grid_size.x as i32..=grid_layout.grid_size.x as i32) {
        let y_low_offset = if q < 0 { -q } else { 0 };
        let y_high_offset = if q > 0 { -q } else { 0 };
        for r in -grid_layout.grid_size.y as i32 + y_low_offset
            ..=grid_layout.grid_size.y as i32 + y_high_offset
        {
            let hex = Hex::from_axial(q, r);
            let pixel_pos = hex_to_pixel(&grid_layout, hex);
            hexes.push((
                hex,
                Mesh3d(meshes.add(hexagon.clone())),
                MeshMaterial3d(transparent_material.clone()),
                Transform::from_translation(Vec3::new(pixel_pos.x, 0.0, pixel_pos.y))
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2))
                    .with_scale(Vec3::splat(0.98)),
            ))
        }
    }
    let entities: Vec<Entity> = world.spawn_batch(hexes).collect();
    for entity in entities {
        world
            .entity_mut(entity)
            .observe(update_material_on::<Pointer<Over>>(base_material.clone()))
            .observe(update_material_on::<Pointer<Out>>(
                transparent_material.clone(),
            ));
    }
}

fn update_material_on<E>(
    new_material: Handle<StandardMaterial>,
) -> impl Fn(Trigger<E>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
    move |trigger, mut query| {
        if let Ok(mut material) = query.get_mut(trigger.target()) {
            material.0 = new_material.clone();
        }
    }
}

pub fn create_hex_polygon_from_layout(layout: &GridLayout) -> ConvexPolygon<6> {
    let unit_hex = Hex::new(0, 0, 0);
    let corners = polygon_corners(layout, unit_hex);
    let polygon = ConvexPolygon::new(corners);
    polygon.unwrap()
}
