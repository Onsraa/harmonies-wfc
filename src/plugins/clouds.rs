use crate::resources::clouds::CloudSettings;
use crate::systems::clouds::animation::{animate_clouds, update_cloud_billboards};
use crate::systems::clouds::interaction::clouds_affect_boids;
use crate::systems::clouds::spawn::{setup_cloud_assets, spawn_clouds};
use crate::systems::menu::GameState;
use bevy::prelude::*;

pub struct CloudsPlugin;

impl Plugin for CloudsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CloudSettings>()
            .add_systems(Startup, setup_cloud_assets)
            .add_systems(
                OnEnter(GameState::InGame),
                spawn_clouds.after(setup_cloud_assets),
            )
            .add_systems(
                Update,
                (animate_clouds, update_cloud_billboards, clouds_affect_boids)
                    .chain()
                    .run_if(in_state(GameState::InGame)),
            );
    }
}
