use crate::resources::camera::CameraSettings;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;

pub fn orbit(
    mut camera: Single<&mut Transform, With<Camera>>,
    camera_settings: Res<CameraSettings>,
    mouse_motion: Res<AccumulatedMouseMotion>,
) {
    let delta = mouse_motion.delta;

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
