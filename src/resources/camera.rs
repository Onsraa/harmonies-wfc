use bevy::prelude::Resource;
use std::f32::consts::FRAC_PI_2;
use std::ops::Range;

#[derive(Debug, Resource)]
pub struct CameraSettings {
    pub pitch_speed: f32,
    pub pitch_range: Range<f32>,
    pub yaw_speed: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        // Limiting pitch stops some unexpected rotation past 90° up or down.
        let pitch_limit = FRAC_PI_2 - 0.01;
        Self {
            pitch_speed: 0.003,
            pitch_range: -pitch_limit..pitch_limit,
            yaw_speed: 0.004,
        }
    }
}
