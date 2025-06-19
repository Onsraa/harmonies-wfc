use crate::components::grid::hex::Hex;
use crate::components::grid::Orientation;
use crate::resources::grid::GridLayout;
use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::{Component, Transform};
use std::f32::consts::PI;

#[derive(Component)]
pub struct HexInstances {
    pub transforms: Vec<Transform>,
    pub colors: Vec<Color>,
}

#[inline]
pub fn hex_to_pixel(layout: &GridLayout, hex: Hex) -> Vec2 {
    let Orientation { f0, f1, f2, f3, .. } = layout.orientation;
    let x = (f0 * hex.q() as f32 + f1 * hex.r() as f32) * layout.hex_size;
    let y = (f2 * hex.q() as f32 + f3 * hex.r() as f32) * layout.hex_size;
    Vec2::new(x, y) + layout.origin
}

#[inline]
fn hex_corner_offset(layout: &GridLayout, corner: usize) -> Vec2 {
    let angle = 2.0 * PI * (corner as f32 + layout.orientation.start_angle) / 6.0;
    Vec2::new(layout.hex_size * angle.cos(), layout.hex_size * angle.sin())
}

#[inline]
pub fn polygon_corners(layout: &GridLayout, hex: Hex) -> [Vec2; 6] {
    let mut corners = [Vec2::ZERO; 6];
    for i in 0..6 {
        corners[i] = hex_to_pixel(&layout, hex) + hex_corner_offset(&layout, i);
    }
    corners
}
