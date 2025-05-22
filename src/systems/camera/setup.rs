use bevy::prelude::{Camera3d, Commands};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3d::default());
}