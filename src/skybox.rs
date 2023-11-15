use bevy::{
    core_pipeline::Skybox,
    input::mouse::MouseMotion,
    prelude::*,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
};
use std::f32::consts::PI;

use crate::{camera::CameraController, loading::Skyboxes, GameState};

pub struct ThirdDimensionPlugin;

impl Plugin for ThirdDimensionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                Update,
                (
                    cycle_cubemap_asset,
                    asset_loaded.after(cycle_cubemap_asset),
                    camera_controller,
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

    // let skybox_handle = asset_server.load("textures/skyboxes/ForbiddenCity/cubemap.png");
    let skybox_handle = asset_server.city.clone();
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraController::default(),
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
            0 => {
                info!("Changing to the City!");
                // asset_server.load("textures/skyboxes/ForbiddenCity/cubemap.png")
                asset_server.city.clone()
            }
            1 => {
                info!("Changing to the Church!");
                // asset_server.load("textures/skyboxes/SaintPetersBasilica/cubemap.png")
                asset_server.church.clone()
            }
            2 => {
                info!("Changing to the Forest!");
                // asset_server.load("textures/skyboxes/MountainPath/cubemap.png")
                asset_server.forest.clone()
            }
            3 => {
                info!("Changing to the Town Square!");
                // asset_server.load("textures/skyboxes/Tallinn/cubemap.png")
                asset_server.town_square.clone()
            }
            4 => {
                info!("Changing to the Mountains!");
                // asset_server.load("textures/skyboxes/Brudslojan/cubemap.png")
                asset_server.mountainside.clone()
            }
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

pub fn camera_controller(
    time: Res<Time>,
    mut mouse_events: EventReader<MouseMotion>,
    mouse_button_input: Res<Input<MouseButton>>,
    key_input: Res<Input<KeyCode>>,
    mut move_toggled: Local<bool>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
) {
    let dt = time.delta_seconds();

    if let Ok((mut transform, mut options)) = query.get_single_mut() {
        if !options.initialized {
            let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
            options.yaw = yaw;
            options.pitch = pitch;
            options.initialized = true;
        }
        if !options.enabled {
            return;
        }

        // Handle key input
        let mut axis_input = Vec3::ZERO;
        if key_input.pressed(options.key_forward) {
            axis_input.z += 1.0;
        }
        if key_input.pressed(options.key_back) {
            axis_input.z -= 1.0;
        }
        if key_input.pressed(options.key_right) {
            axis_input.x += 1.0;
        }
        if key_input.pressed(options.key_left) {
            axis_input.x -= 1.0;
        }
        if key_input.pressed(options.key_up) {
            axis_input.y += 1.0;
        }
        if key_input.pressed(options.key_down) {
            axis_input.y -= 1.0;
        }
        if key_input.just_pressed(options.keyboard_key_enable_mouse) {
            *move_toggled = !*move_toggled;
        }

        // Apply movement update
        if axis_input != Vec3::ZERO {
            let max_speed = if key_input.pressed(options.key_run) {
                options.run_speed
            } else {
                options.walk_speed
            };
            options.velocity = axis_input.normalize() * max_speed;
        } else {
            let friction = options.friction.clamp(0.0, 1.0);
            options.velocity *= 1.0 - friction;
            if options.velocity.length_squared() < 1e-6 {
                options.velocity = Vec3::ZERO;
            }
        }
        let forward = transform.forward();
        let right = transform.right();
        transform.translation += options.velocity.x * dt * right
            + options.velocity.y * dt * Vec3::Y
            + options.velocity.z * dt * forward;

        // Handle mouse input
        let mut mouse_delta = Vec2::ZERO;
        if mouse_button_input.pressed(options.mouse_key_enable_mouse) || *move_toggled {
            for mouse_event in mouse_events.read() {
                mouse_delta += mouse_event.delta;
            }
        }

        if mouse_delta != Vec2::ZERO {
            // Apply look update
            options.pitch = (options.pitch - mouse_delta.y * 0.5 * options.sensitivity * dt)
                .clamp(-PI / 2., PI / 2.);
            options.yaw -= mouse_delta.x * options.sensitivity * dt;
            transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, options.yaw, options.pitch);
        }
    }
}
