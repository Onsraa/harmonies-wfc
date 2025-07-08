use bevy::prelude::*;

#[derive(Component)]
pub struct Cloud {
    pub base_position: Vec3,
    pub drift_speed: Vec3,
    pub oscillation_offset: f32,
}

#[derive(Component)]
pub struct CloudBillboard;