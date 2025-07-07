use bevy::prelude::*;

#[derive(Component)]
pub struct Boid {
    pub group: u8,
}

#[derive(Component)]
pub struct Velocity {
    pub velocity: Vec3,
}

#[derive(Component)]
pub struct Acceleration {
    pub acceleration: Vec3,
}

#[derive(Component)]
pub struct Obstacle {
    pub radius: f32,
}