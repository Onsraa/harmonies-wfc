use crate::components::boid::{Acceleration, Boid, Velocity};
use crate::components::spatial::TrackedByKDTree3D;
use crate::globals::{BOID_MAX_HEIGHT, BOID_MIN_HEIGHT, BOID_ZONE_SIZE};
use crate::resources::boids::BoidSettings;
use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

pub fn spawn_boids(
    mut commands: Commands,
    boid_settings: Res<BoidSettings>,
    asset_server: Res<AssetServer>,
) {
    for _ in 0..boid_settings.count {
        spawn_boid_entity(&mut commands, &boid_settings, &asset_server);
    }
}

fn spawn_boid_entity(
    commands: &mut Commands,
    boid_settings: &BoidSettings,
    asset_server: &Res<AssetServer>,
) {
    let mut rng = rand::rng();
    let group = rng.random_range(0..boid_settings.group_count) as u8;

    let random_pos = Vec3::new(
        rng.random_range(-BOID_ZONE_SIZE * 0.45..BOID_ZONE_SIZE * 0.45),
        rng.random_range(BOID_MIN_HEIGHT..BOID_MAX_HEIGHT * 0.95),
        rng.random_range(-BOID_ZONE_SIZE * 0.45..BOID_ZONE_SIZE * 0.45),
    );

    // Vitesse initiale aléatoire
    let theta = rng.random_range(0.0..2.0 * PI);
    let phi = rng.random_range(0.0..PI);
    let initial_velocity = Vec3::new(
        f32::sin(phi) * f32::cos(theta),
        f32::sin(phi) * f32::sin(theta),
        f32::cos(phi),
    ) * boid_settings.min_speed;

    commands.spawn((
        Boid { group },
        Velocity { velocity: initial_velocity },
        Acceleration { acceleration: Vec3::ZERO },
        TrackedByKDTree3D,
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/bird.gltf"))),
        Transform {
            translation: random_pos,
            scale: Vec3::splat(boid_settings.size),
            ..default()
        },
    ));
}