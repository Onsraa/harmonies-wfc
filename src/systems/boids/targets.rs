use crate::resources::boids::{GroupsTargets, BoidSettings};
use crate::globals::{BOID_ZONE_SIZE, BOID_MIN_HEIGHT, BOID_MAX_HEIGHT, TILE_HEIGHT, TILE_GAP};
use crate::components::grid::tile::Tile;
use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

#[derive(Resource)]
pub struct TargetMovementTimer {
    timer: Timer,
    waypoint_timer: Timer,
}

impl Default for TargetMovementTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
            waypoint_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
pub struct TargetWaypoint {
    pub position: Vec3,
    pub velocity: Vec3,
    pub mode: MovementMode,
}

#[derive(Clone, Copy)]
pub enum MovementMode {
    Random,
    NearTiles,
    HighAltitude,
    LowSwoop,
}

pub fn move_targets_system(
    mut groups_targets: ResMut<GroupsTargets>,
    boid_settings: Res<BoidSettings>,
    mut timer: ResMut<TargetMovementTimer>,
    time: Res<Time>,
    tile_query: Query<&Transform, With<Tile>>,
) {
    timer.timer.tick(time.delta());
    timer.waypoint_timer.tick(time.delta());

    // Changer complètement de zone toutes les 5 secondes
    if timer.timer.just_finished() {
        let mut rng = rand::rng();

        // S'assurer qu'on a le bon nombre de cibles
        if groups_targets.targets.len() as u8 != boid_settings.group_count {
            *groups_targets = GroupsTargets::new(boid_settings.group_count);
        }

        // Collecter les positions des tuiles pour potentiellement s'en approcher
        let tile_positions: Vec<Vec3> = tile_query.iter()
            .map(|t| t.translation)
            .collect();

        for (i, target) in groups_targets.targets.iter_mut().enumerate() {
            let mode = choose_movement_mode(&mut rng);

            match mode {
                MovementMode::Random => {
                    // Position complètement aléatoire
                    *target = Vec3::new(
                        rng.random_range(-BOID_ZONE_SIZE ..BOID_ZONE_SIZE ),
                        rng.random_range(BOID_MIN_HEIGHT..BOID_MAX_HEIGHT),
                        rng.random_range(-BOID_ZONE_SIZE ..BOID_ZONE_SIZE ),
                    );
                }
                MovementMode::NearTiles => {
                    // Se positionner près d'une tuile aléatoire
                    if !tile_positions.is_empty() {
                        let tile_pos = tile_positions[rng.random_range(0..tile_positions.len())];
                        let offset = Vec3::new(
                            rng.random_range(-10.0..10.0),
                            rng.random_range(5.0..15.0),
                            rng.random_range(-10.0..10.0),
                        );
                        *target = tile_pos + offset;

                        // Limiter aux bornes
                        target.x = target.x.clamp(-BOID_ZONE_SIZE * 0.45, BOID_ZONE_SIZE * 0.45);
                        target.y = target.y.clamp(BOID_MIN_HEIGHT, BOID_MAX_HEIGHT);
                        target.z = target.z.clamp(-BOID_ZONE_SIZE * 0.45, BOID_ZONE_SIZE * 0.45);
                    }
                }
                MovementMode::HighAltitude => {
                    // Vol en haute altitude
                    *target = Vec3::new(
                        rng.random_range(-BOID_ZONE_SIZE * 0.3..BOID_ZONE_SIZE * 0.3),
                        rng.random_range(BOID_MAX_HEIGHT * 0.7..BOID_MAX_HEIGHT * 0.95),
                        rng.random_range(-BOID_ZONE_SIZE * 0.3..BOID_ZONE_SIZE * 0.3),
                    );
                }
                MovementMode::LowSwoop => {
                    // Vol rasant
                    *target = Vec3::new(
                        rng.random_range(-BOID_ZONE_SIZE * 0.4..BOID_ZONE_SIZE * 0.4),
                        rng.random_range(BOID_MIN_HEIGHT..BOID_MIN_HEIGHT + 15.0),
                        rng.random_range(-BOID_ZONE_SIZE * 0.4..BOID_ZONE_SIZE * 0.4),
                    );
                }
            }
        }
    }
}

pub fn smooth_target_movement_system(
    mut groups_targets: ResMut<GroupsTargets>,
    time: Res<Time>,
    tile_query: Query<&Transform, With<Tile>>,
) {
    let base_speed = 20.0;
    let mut rng = rand::rng();

    // Collecter les positions des tuiles pour l'attraction occasionnelle
    let tile_positions: Vec<Vec3> = tile_query.iter()
        .map(|t| t.translation)
        .take(20) // Limiter pour la performance
        .collect();

    for (i, target) in groups_targets.targets.iter_mut().enumerate() {
        // Mouvement de base avec des patterns variés
        let time_offset = i as f32 * 1.5;
        let t = time.elapsed_secs() + time_offset;

        // Créer des mouvements sinusoïdaux complexes
        let horizontal_pattern = Vec3::new(
            f32::sin(t * 0.3) * 5.0 + f32::cos(t * 0.7) * 3.0,
            0.0,
            f32::cos(t * 0.4) * 5.0 + f32::sin(t * 0.6) * 3.0,
        );

        // Mouvement vertical avec des plongées et montées
        let vertical_amplitude = 15.0;
        let vertical_pattern = f32::sin(t * 0.5) * vertical_amplitude
            + f32::sin(t * 1.2) * vertical_amplitude * 0.3;

        // Ajouter de l'aléatoire
        let random_movement = Vec3::new(
            rng.random_range(-2.0..2.0),
            rng.random_range(-3.0..3.0),
            rng.random_range(-2.0..2.0),
        );

        // Parfois, attirer vers une tuile proche
        let mut tile_attraction = Vec3::ZERO;
        if rng.random::<f32>() < 0.1 && !tile_positions.is_empty() {
            // Trouver la tuile la plus proche
            if let Some(closest_tile) = tile_positions.iter()
                .min_by_key(|&&tile_pos| ((*target - tile_pos).length() * 100.0) as i32) {
                let to_tile = *closest_tile + Vec3::Y * 10.0 - *target;
                if to_tile.length() < 30.0 {
                    tile_attraction = to_tile.normalize() * 5.0;
                }
            }
        }

        // Combiner tous les mouvements
        let total_movement = (horizontal_pattern + random_movement + tile_attraction) * time.delta_secs();
        *target += total_movement;
        target.y += vertical_pattern * time.delta_secs() * 0.5;

        // Garder les cibles dans les limites avec des marges différentes selon l'altitude
        let margin = if target.y > BOID_MAX_HEIGHT * 0.7 { 0.3 } else { 0.45 };
        target.x = target.x.clamp(-BOID_ZONE_SIZE * margin, BOID_ZONE_SIZE * margin);
        target.y = target.y.clamp(BOID_MIN_HEIGHT, BOID_MAX_HEIGHT);
        target.z = target.z.clamp(-BOID_ZONE_SIZE * margin, BOID_ZONE_SIZE * margin);

        // Éviter que les cibles ne rentrent dans les tuiles
        for tile_pos in &tile_positions {
            let to_target = *target - *tile_pos;
            if to_target.length() < 5.0 {
                *target = *tile_pos + to_target.normalize() * 5.0;
            }
        }
    }
}

fn choose_movement_mode(rng: &mut impl Rng) -> MovementMode {
    let choice = rng.random::<f32>();
    if choice < 0.3 {
        MovementMode::Random
    } else if choice < 0.5 {
        MovementMode::NearTiles
    } else if choice < 0.75 {
        MovementMode::HighAltitude
    } else {
        MovementMode::LowSwoop
    }
}

// Système optionnel pour visualiser les cibles (debug)
pub fn debug_render_targets(
    mut gizmos: Gizmos,
    groups_targets: Res<GroupsTargets>,
    boid_settings: Res<BoidSettings>,
) {
    let colors = [
        Color::srgb(1.0, 0.0, 0.0),
        Color::srgb(0.0, 1.0, 0.0),
        Color::srgb(0.0, 0.0, 1.0),
        Color::srgb(1.0, 1.0, 0.0),
        Color::srgb(1.0, 0.0, 1.0),
        Color::srgb(0.0, 1.0, 1.0),
        Color::srgb(1.0, 0.5, 0.0),
        Color::srgb(0.5, 0.0, 1.0),
        Color::srgb(0.0, 0.5, 0.5),
        Color::srgb(0.5, 0.5, 0.5),
    ];

    for (i, target) in groups_targets.targets.iter().enumerate() {
        let color = colors[i % colors.len()];
        gizmos.sphere(*target, 2.0, color);
    }
}