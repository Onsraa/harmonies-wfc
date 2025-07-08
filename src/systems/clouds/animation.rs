use crate::components::clouds::{Cloud, CloudBillboard};
use crate::globals::BOID_ZONE_SIZE;
use crate::resources::clouds::CloudSettings;
use bevy::prelude::*;

pub fn animate_clouds(
    time: Res<Time>,
    cloud_settings: Res<CloudSettings>,
    mut query: Query<(&mut Transform, &Cloud)>,
) {
    let elapsed = time.elapsed_secs();

    for (mut transform, cloud) in query.iter_mut() {
        // Dérive horizontale
        transform.translation += cloud.drift_speed * time.delta_secs();

        // Oscillation verticale douce
        let vertical_offset = (elapsed * cloud_settings.oscillation_speed + cloud.oscillation_offset).sin()
            * cloud_settings.oscillation_amplitude;
        transform.translation.y = cloud.base_position.y + vertical_offset;

        // Rotation très lente pour du dynamisme subtil
        transform.rotate_y(time.delta_secs() * 0.02);

        // Wrap around pour créer un cycle infini
        let boundary = BOID_ZONE_SIZE * 1.2;

        if transform.translation.x > boundary {
            transform.translation.x -= boundary * 2.0;
        } else if transform.translation.x < -boundary {
            transform.translation.x += boundary * 2.0;
        }

        if transform.translation.z > boundary {
            transform.translation.z -= boundary * 2.0;
        } else if transform.translation.z < -boundary {
            transform.translation.z += boundary * 2.0;
        }
    }
}

pub fn update_cloud_billboards(
    camera_query: Query<&Transform, With<Camera>>,
    mut cloud_query: Query<&mut Transform, (With<CloudBillboard>, Without<Camera>)>,
) {
    if let Ok(camera_transform) = camera_query.get_single() {
        for mut cloud_transform in cloud_query.iter_mut() {
            // Calculer la direction vers la caméra
            let to_camera = (camera_transform.translation - cloud_transform.translation)
                .normalize();

            // Garder l'axe Y vertical
            let right = to_camera.cross(Vec3::Y).normalize();
            let up = right.cross(to_camera).normalize();

            // Appliquer la rotation en préservant l'échelle
            let scale = cloud_transform.scale;
            cloud_transform.look_to(to_camera, up);
            cloud_transform.scale = scale;
        }
    }
}