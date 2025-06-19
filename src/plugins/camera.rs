use crate::resources::camera::CameraSettings;
use crate::systems::camera::{rotate, set_elevation, setup_camera, smooth_elevation, translate};
use crate::systems::menu::GameState;
use crate::systems::window::{hide_cursor, show_cursor};
use bevy::input::common_conditions::{input_just_released, input_pressed};
use bevy::prelude::{in_state, App, Condition, IntoScheduleConfigs, MouseButton, Plugin, Startup, Update};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraSettings>()
            .add_systems(Startup, setup_camera)
            .add_systems(Update, (rotate, hide_cursor).run_if(input_pressed(MouseButton::Right).and(in_state(GameState::InGame))))
            .add_systems(Update, show_cursor.run_if(input_just_released(MouseButton::Right).and(in_state(GameState::InGame))))
            .add_systems(Update, (set_elevation, smooth_elevation).run_if(in_state(GameState::InGame)))
            .add_systems(Update, translate.run_if(in_state(GameState::InGame)));
    }
}
