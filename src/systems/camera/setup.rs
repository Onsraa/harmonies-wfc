use bevy::{
    core_pipeline::Skybox,
    prelude::*,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
};

#[derive(Resource)]
pub struct SkyboxImage {
    is_loaded: bool,
    image_handle: Handle<Image>,
}

pub fn setup_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    let skybox_handle = asset_server.load("images/skybox.png");

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
    ));

    commands.insert_resource(SkyboxImage {
        is_loaded: false,
        image_handle: skybox_handle,
    });
}

pub fn load_skybox(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut skybox_image: ResMut<SkyboxImage>,
    mut cameras: Query<Entity, (With<Camera3d>, Without<Skybox>)>,
) {
    if !skybox_image.is_loaded
        && asset_server
            .load_state(&skybox_image.image_handle)
            .is_loaded()
    {
        if let Some(image) = images.get_mut(&skybox_image.image_handle) {
            println!("Image dimensions: {}x{}", image.width(), image.height());
            println!(
                "Aspect ratio: {}",
                image.height() as f32 / image.width() as f32
            );

            // Convert regular 2D image to cubemap format
            if image.texture_descriptor.array_layer_count() == 1 {
                image.reinterpret_stacked_2d_as_array(image.height() / image.width());
                image.texture_view_descriptor = Some(TextureViewDescriptor {
                    dimension: Some(TextureViewDimension::Cube),
                    ..default()
                });
            }

            // Now add the skybox to the camera
            for camera_entity in cameras.iter() {
                commands.entity(camera_entity).insert(Skybox {
                    image: skybox_image.image_handle.clone(),
                    brightness: 1000.0,
                    ..default()
                });
            }

            skybox_image.is_loaded = true;
            println!("Skybox loaded and applied!");
        }
    }
}
