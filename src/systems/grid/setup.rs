use crate::components::grid::hex::display::hex_to_pixel;
use crate::components::grid::hex::Hex;
use crate::resources::grid::GridLayout;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;

pub fn setup_grid(world: &mut World, params: &mut SystemState<Res<GridLayout>>) {
    let grid_layout = params.get_mut(world);
    let mut hexes: Vec<_> = vec![];
    for q in -grid_layout.width..=grid_layout.width {
        let y_low_offset = if q < 0 { -q } else { 0 };
        let y_high_offset = if q > 0 { -q } else { 0 };
        for r in -grid_layout.length + y_low_offset..=grid_layout.length + y_high_offset {
            for level in 0..grid_layout.max_height {
                let hex = Hex::from_axial(q, r, level as i32);
                let pixel_pos = hex_to_pixel(&grid_layout, hex);
                hexes.push((
                    hex,
                    Transform::from_translation(Vec3::new(pixel_pos.x, 0.0, pixel_pos.y))
                        .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2))
                        .with_scale(Vec3::splat(0.98)),
                ))
            }
        }
    }
    world.spawn_batch(hexes);
}
