use crate::components::spatial::TrackedByKDTree3D;
use bevy::prelude::*;
use bevy_spatial::{AutomaticUpdate, SpatialStructure, TransformMode};
use std::time::Duration;

pub struct SpatialPlugin;

impl Plugin for SpatialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            AutomaticUpdate::<TrackedByKDTree3D>::new()
                .with_spatial_ds(SpatialStructure::KDTree3)
                .with_frequency(Duration::from_millis(10))
                .with_transform(TransformMode::Transform),
        );
    }
}