use crate::resources::boids::{BoidSettings, GroupsTargets};
use crate::systems::boids::spawn::spawn_boids;
use crate::systems::boids::flocking::*;
use crate::systems::boids::targets::{debug_render_targets, move_targets_system, smooth_target_movement_system, TargetMovementTimer};
use crate::events::ApplyForceEvent;
use crate::systems::menu::GameState;
use bevy::prelude::*;

pub struct BoidsPlugin;

impl Plugin for BoidsPlugin {
    fn build(&self, app: &mut App) {
        let boid_settings = BoidSettings::default();
        let group_count = boid_settings.group_count;

        app.insert_resource(boid_settings)
            .insert_resource(GroupsTargets::new(group_count))
            .insert_resource(TargetMovementTimer::default())
            .add_event::<ApplyForceEvent>()
            .add_systems(OnEnter(GameState::InGame), spawn_boids)
            .add_systems(Update, (
                move_targets_system,
                smooth_target_movement_system,
                flocking_system,
                avoid_obstacles,
                apply_forces,
                update_boids,
                confine_boids,
                debug_render_targets,
            ).chain().run_if(in_state(GameState::InGame)));
    }
}