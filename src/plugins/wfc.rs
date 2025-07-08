use crate::systems::wfc::v2::wfc::{
    cancel_wfc_task, check_wfc_progress, reset_grid_system, setup_wfc_task_when_ready, start_wfc_task,
    trigger_grid_reset, ResetGrid, WfcStatus,
};
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

pub struct WfcPlugin;

impl Plugin for WfcPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WfcStatus>();
        app.add_event::<ResetGrid>();

        app.add_systems(
            Update,
            (
                setup_wfc_task_when_ready,
                check_wfc_progress,
                reset_grid_system,
            )
                .chain(),
        );

        // Space to run WFC
        app.add_systems(
            Update,
            start_wfc_task.run_if(input_just_pressed(KeyCode::Space)),
        );

        // R to reset grid
        app.add_systems(
            Update,
            trigger_grid_reset.run_if(input_just_pressed(KeyCode::KeyR)),
        );

        // Escape to cancel running task
        app.add_systems(
            Update,
            cancel_wfc_task.run_if(input_just_pressed(KeyCode::Escape)),
        );
    }
}
