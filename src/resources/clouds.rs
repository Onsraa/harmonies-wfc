use bevy::prelude::*;
use crate::globals::{BOID_MAX_HEIGHT, BOID_MIN_HEIGHT};

#[derive(Resource)]
pub struct CloudSettings {
    pub cloud_count: usize,
    pub min_height: f32,
    pub max_height: f32,
    pub opacity: f32,
    pub cloud_scale: f32,
    pub wind_speed: Vec3,
    pub oscillation_amplitude: f32,
    pub oscillation_speed: f32,
    pub affect_boids: bool,
    pub turbulence_strength: f32,
}

impl Default for CloudSettings {
    fn default() -> Self {
        Self {
            cloud_count: 60,
            min_height: BOID_MIN_HEIGHT * 1.3,
            max_height: BOID_MAX_HEIGHT,
            opacity: 0.4,
            cloud_scale: 20.0,
            wind_speed: Vec3::new(1.5, 0.0, 0.3),
            oscillation_amplitude: 2.0,
            oscillation_speed: 0.2,
            affect_boids: true,
            turbulence_strength: 0.3,
        }
    }
}

#[derive(Resource)]
pub struct CloudAssets {
    pub cloud_texture: Handle<Image>,
    pub cloud_mesh: Handle<Mesh>,
}