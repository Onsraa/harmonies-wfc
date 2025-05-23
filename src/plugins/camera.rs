use crate::resources::camera::CameraSettings;
use crate::systems::camera::{orbit, setup_camera};
use crate::systems::window::{hide_cursor, show_cursor};
use bevy::input::common_conditions::{input_just_released, input_pressed};
use bevy::prelude::{App, IntoScheduleConfigs, MouseButton, Plugin, Startup, Update};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraSettings>()
            .add_systems(Startup, setup_camera)
            .add_systems(
                Update,
                (orbit, hide_cursor).run_if(input_pressed(MouseButton::Right)),
            )
            .add_systems(
                Update,
                show_cursor.run_if(input_just_released(MouseButton::Right)),
            );
    }
}
