use bevy::{
    core_pipeline::Skybox,
    prelude::*,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
};
use bevy_third_person_camera::{CameraFocusModifier, Offset, ThirdPersonCamera, Zoom};
use std::f32::consts::{E, PI};

use crate::{loading::Skyboxes, GameState};

pub struct ThirdDimensionPlugin;

impl Plugin for ThirdDimensionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                Update,
                (
                    cycle_cubemap_asset,
                    asset_loaded.after(cycle_cubemap_asset),
                    animate_light_direction,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Resource)]
struct Cubemap {
    is_loaded: bool,
    index: u8,
    image_handle: Handle<Image>,
}

// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
fn setup(mut commands: Commands, asset_server: Res<Skyboxes>) {
    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 32000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 2.0, 0.0)
            .with_rotation(Quat::from_rotation_x(-PI / 4.)),
        ..default()
    });

    let skybox_handle = asset_server.city.clone();
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 8.0)
                .looking_at(Vec3::new(0., 0.81, 0.), Vec3::Y),
            ..default()
        },
        ThirdPersonCamera {
            true_focus: Vec3::new(0., 0.81, 0.),
            aim_enabled: true,
            aim_zoom: 0.7,
            zoom_enabled: false,
            zoom: Zoom::new(1.5, 5.0),
            offset_enabled: true,
            offset: Offset::new(0.4, 0.0),
            focus_modifier: Some(CameraFocusModifier {
                lower_threshold: PI / 2.,
                upper_threshold: 2. * PI / 3.,
                max_forward_displacement: 0.5,
                max_backward_displacement: 0.5,
                // typical logistic function centered at 0.5
                lower_displacement_function: |x| 1. / (1. + E.powf(-15. * (x - 0.5))),
                upper_displacement_function: |x| 1. / (1. + E.powf(-15. * (x - 0.5))),
                behind_radius_displacement: 2.0,
                lower_radius_function: |x| 1. - E.powf(-4. * x),
            }),
            ..default()
        },
        Skybox(skybox_handle.clone()),
    ));

    // ambient light
    // NOTE: The ambient light is used to scale how bright the environment map is so with a bright
    // environment map, use an appropriate color and brightness to match
    commands.insert_resource(AmbientLight {
        color: Color::rgb_u8(210, 220, 240),
        brightness: 1.0,
    });

    commands.insert_resource(Cubemap {
        is_loaded: false,
        index: 0,
        image_handle: skybox_handle,
    });
}

fn cycle_cubemap_asset(
    mut cubemap: ResMut<Cubemap>,
    asset_server: Res<Skyboxes>,
    key_input: Res<Input<KeyCode>>,
) {
    if key_input.just_pressed(KeyCode::Space) {
        let mut new_index = rand::random::<u8>() % 5;
        while new_index == cubemap.index {
            new_index = rand::random::<u8>() % 5;
        }

        cubemap.index = new_index;
        cubemap.image_handle = match new_index {
            0 => asset_server.city.clone(),
            1 => asset_server.church.clone(),
            2 => asset_server.forest.clone(),
            3 => asset_server.town_square.clone(),
            4 => asset_server.mountainside.clone(),
            _ => unreachable!(),
        };
        cubemap.is_loaded = false;
    }
}

fn asset_loaded(
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
    mut skyboxes: Query<&mut Skybox>,
) {
    if !cubemap.is_loaded {
        let image = images.get_mut(&cubemap.image_handle).unwrap();
        // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
        // so they appear as one texture. The following code reconfigures the texture as necessary.
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(image.height() / image.width());
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }

        for mut skybox in &mut skyboxes {
            skybox.0 = cubemap.image_handle.clone();
        }

        cubemap.is_loaded = true;
    }
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() * 0.5);
    }
}
