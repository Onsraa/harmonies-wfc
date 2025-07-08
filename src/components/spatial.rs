use bevy::prelude::*;
use bevy_spatial::kdtree::KDTree3;

#[derive(Component, Default)]
pub struct TrackedByKDTree3D;

#[derive(Component, Default)]
pub struct ObstacleInKDTree;

pub type NNTree3D = KDTree3<TrackedByKDTree3D>;
pub type ObstacleTree3D = KDTree3<ObstacleInKDTree>;
