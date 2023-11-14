use bevy::{
    core_pipeline::Skybox,
    prelude::*,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
};

use crate::{camera::CameraController, loading::Skyboxes};

#[derive(Resource)]
pub struct Cubemap {
    is_loaded: bool,
    handle: Handle<Image>,
}

fn setup_skybox(mut commands: Commands, skyboxes: Res<Skyboxes>) {
    let skybox_handle = &skyboxes.city;
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(CameraController::default());
    commands.spawn(Skybox(skybox_handle.clone()));
    commands.insert_resource(Cubemap {
        is_loaded: false,
        handle: skybox_handle.clone(),
    });
}

fn skybox_loaded(
    mut cubemap: ResMut<Cubemap>,
    mut images: ResMut<Assets<Image>>,
    mut skyboxes: Query<&mut Skybox>,
) {
    if !cubemap.is_loaded {
        let image = images.get_mut(&cubemap.handle).unwrap();
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(image.height() / image.width());
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }

        for mut skybox in &mut skyboxes {
            skybox.0 = cubemap.handle.clone();
        }
        cubemap.is_loaded = true;
    }
}
