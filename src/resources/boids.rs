use bevy::prelude::*;
use crate::globals::GROUP_COUNT;

#[derive(Resource)]
pub struct BoidSettings {
    pub count: usize,
    pub size: f32,
    pub cohesion_range: f32,
    pub alignment_range: f32,
    pub separation_range: f32,
    pub min_distance_between_boids: f32,
    pub cohesion_coeff: f32,
    pub alignment_coeff: f32,
    pub separation_coeff: f32,
    pub collision_coeff: f32,
    pub min_speed: f32,
    pub max_speed: f32,
    pub bounce_against_walls: bool,
    pub attraction_coeff: f32,
    pub field_of_view: f32,
    pub tile_avoidance_range: f32,
    pub group_count: u8,
}

impl Default for BoidSettings {
    fn default() -> Self {
        BoidSettings {
            count: 500,
            size: 0.05,
            cohesion_range: 30.0,
            alignment_range: 20.0,
            separation_range: 3.0,
            min_distance_between_boids: 10.0,
            cohesion_coeff: 20.0,
            alignment_coeff: 5.0,
            separation_coeff: 40.0,
            collision_coeff: 30.0,
            min_speed: 10.0,
            max_speed: 40.0,
            bounce_against_walls: true,
            attraction_coeff: 1.0,
            field_of_view: 90.0,
            tile_avoidance_range: 8.0,
            group_count: GROUP_COUNT,
        }
    }
}

#[derive(Resource)]
pub struct GroupsTargets {
    pub targets: Vec<Vec3>,
}

impl GroupsTargets {
    pub fn new(group_count: u8) -> Self {
        use crate::globals::{BOID_ZONE_SIZE, BOID_MIN_HEIGHT, BOID_MAX_HEIGHT};
        use std::f32::consts::PI;

        let mut targets = Vec::new();

        for i in 0..group_count {
            let angle = (i as f32 / group_count as f32) * 2.0 * PI;
            let radius = BOID_ZONE_SIZE * 0.3;

            targets.push(Vec3::new(
                angle.cos() * radius,
                BOID_MIN_HEIGHT + (BOID_MAX_HEIGHT - BOID_MIN_HEIGHT) * 0.5,
                angle.sin() * radius,
            ));
        }

        GroupsTargets { targets }
    }
}

impl Default for GroupsTargets {
    fn default() -> Self {
        Self::new(GROUP_COUNT)
    }
}