use crate::resources::camera::CameraSettings;
use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::prelude::*;

pub fn rotate(
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
    let scale_factor = (camera.translation.y * 0.1 + 1.0).ln() + 1.0;
    let delta_elevation = delta.y * camera_settings.elevation_speed * scale_factor;

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

pub fn translate(
    mut camera: Single<&mut Transform, With<Camera>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    camera_settings: Res<CameraSettings>,
    time: Res<Time>,
) {
    let height_scale = (camera.translation.y / 10.).sqrt().max(15.0);
    let speed = camera_settings.translation_speed * time.delta_secs() * height_scale;
    let mut velocity = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        velocity.y += speed;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        velocity.y -= speed;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        velocity.x -= speed;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        velocity.x += speed;
    }

    let forward = camera.forward();
    let horizontal_forward = Vec3::new(forward.x, 0.0, forward.z).normalize();

    let right = camera.right();
    let horizontal_right = Vec3::new(right.x, 0.0, right.z).normalize();

    let translation_vec = horizontal_forward * velocity.y + horizontal_right * velocity.x;
    camera.translation.x += translation_vec.x;
    camera.translation.z += translation_vec.z;
}
