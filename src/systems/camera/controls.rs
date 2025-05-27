use crate::resources::camera::CameraSettings;
use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::prelude::*;

pub fn orbit(
    mut camera: Single<&mut Transform, With<Camera>>,
    camera_settings: Res<CameraSettings>,
    mouse_motion: Res<AccumulatedMouseMotion>,
) {
    let delta = -mouse_motion.delta;

    let delta_pitch = delta.y * camera_settings.pitch_speed;
    let delta_yaw = delta.x * camera_settings.yaw_speed;

    let (yaw, pitch, _) = camera.rotation.to_euler(EulerRot::YXZ);

    let pitch = (pitch + delta_pitch).clamp(
        camera_settings.pitch_range.start,
        camera_settings.pitch_range.end,
    );
    let yaw = yaw + delta_yaw;
    camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
}

pub fn set_elevation(
    camera: Single<&Transform, With<Camera>>,
    mut camera_settings: ResMut<CameraSettings>, // ResMut for mutation
    mouse_scroll: Res<AccumulatedMouseScroll>,
) {
    let delta = -mouse_scroll.delta;
    let delta_elevation = delta.y * camera_settings.elevation_speed.powi(2);

    camera_settings.elevation_goal = Some(
        camera_settings
            .elevation_goal
            .unwrap_or(camera.translation.y)
            + delta_elevation,
    )
    .map(|goal| {
        goal.clamp(
            camera_settings.elevation_range.start,
            camera_settings.elevation_range.end,
        )
    });
}

pub fn smooth_elevation(
    mut camera: Single<&mut Transform, With<Camera>>,
    mut camera_settings: ResMut<CameraSettings>,
    time: Res<Time>,
) {
    if let Some(goal) = camera_settings.elevation_goal {
        let distance = (camera.translation.y - goal).abs();

        if distance < 0.05 {
            camera.translation.y = goal;
            camera_settings.elevation_goal = None;
        } else {
            let decay_rate = 8.0;
            camera.translation.y +=
                (goal - camera.translation.y) * (1.0 - (-decay_rate * time.delta_secs()).exp());
        }
    }
}
