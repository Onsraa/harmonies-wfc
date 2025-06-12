use crate::components::grid::Orientation;
use bevy::math::Vec2;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct GridLayout {
    pub orientation: Orientation,
    pub grid_size: Vec2,
    pub hex_size: f32,
    pub origin: Vec2,
}

impl Default for GridLayout {
    fn default() -> Self {
        GridLayout {
            orientation: Orientation::POINTY,
            grid_size: Vec2::new(10.0, 10.0),
            hex_size: 2.0,
            origin: Vec2::ZERO,
        }
    }
}
