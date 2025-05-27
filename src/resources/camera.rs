use bevy::prelude::Resource;
use std::f32::consts::FRAC_PI_2;
use std::ops::Range;

#[derive(Debug, Resource)]
pub struct CameraSettings {
    pub pitch_speed: f32,
    pub pitch_range: Range<f32>,
    pub yaw_speed: f32,

    pub elevation_speed: f32,
    pub elevation_range: Range<f32>,
    pub elevation_goal: Option<f32>,
    pub elevation_smoothing: f32,

    pub translation_speed: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        // Limiting pitch stops some unexpected rotation past 90° up or down.
        let pitch_limit = FRAC_PI_2 - 0.01;
        Self {
            pitch_speed: 0.003,
            pitch_range: -pitch_limit..pitch_limit,
            yaw_speed: 0.004,

            elevation_speed: 1.,
            elevation_range: 1.0..50.0,
            elevation_goal: None,
            elevation_smoothing: 0.8,

            translation_speed: 10.,
        }
    }
}
