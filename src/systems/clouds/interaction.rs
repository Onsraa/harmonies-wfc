use crate::components::boid::Boid;
use crate::components::clouds::Cloud;
use crate::events::ApplyForceEvent;
use crate::resources::clouds::CloudSettings;
use bevy::prelude::*;
use rand::Rng;

pub fn clouds_affect_boids(
    cloud_settings: Res<CloudSettings>,
    cloud_query: Query<&Transform, With<Cloud>>,
    boid_query: Query<(Entity, &Transform), With<Boid>>,
    mut event_writer: EventWriter<ApplyForceEvent>,
) {
    if !cloud_settings.affect_boids {
        return;
    }

    let mut rng = rand::rng();

    for cloud_transform in cloud_query.iter() {
        for (entity, boid_transform) in boid_query.iter() {
            let to_cloud = cloud_transform.translation - boid_transform.translation;
            let distance = to_cloud.length();

            // Zone d'influence du nuage (plus grande zone, effet plus doux)
            if distance < 30.0 {
                // Turbulence qui diminue avec la distance
                let turbulence_factor = (1.0 - distance / 30.0) * cloud_settings.turbulence_strength;

                let turbulence = Vec3::new(
                    rng.random_range(-1.0..1.0),
                    rng.random_range(-0.3..0.3),
                    rng.random_range(-1.0..1.0),
                ) * turbulence_factor;

                event_writer.write(ApplyForceEvent {
                    entity,
                    force: turbulence,
                });
            }
        }
    }
}