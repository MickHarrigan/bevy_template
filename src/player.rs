use crate::actions::Actions;
use crate::GameState;
use bevy::{prelude::*, render::mesh::VertexAttributeValues};
use bevy_rapier3d::{
    prelude::{
        CharacterAutostep, CharacterLength, Collider, KinematicCharacterController, NoUserData,
        QueryFilter, RapierConfiguration, RapierContext, RapierPhysicsPlugin, RigidBody,
    },
    render::RapierDebugRenderPlugin,
};
use bevy_third_person_camera::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app /*.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default())*/
            .add_plugins(ThirdPersonCameraPlugin)
            .init_resource::<RapierContext>()
            .insert_resource(RapierConfiguration {
                gravity: Vec3::Y * -980.0,
                ..default()
            })
            .add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update,
                (
                    player_movement_keyboard,
                    check_player_collisions, /*, update_gravity*/
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        SceneBundle {
            scene: assets.load("models/Player.gltf#Scene0"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        },
        // RigidBody::KinematicPositionBased,
        RigidBody::Dynamic,
        Collider::cylinder(1.0, 0.5),
        // KinematicCharacterController {
        //     autostep: Some(CharacterAutostep {
        //         max_height: CharacterLength::Relative(0.3),
        //         min_width: CharacterLength::Relative(0.5),
        //         include_dynamic_bodies: false,
        //     }),
        //     // snap_to_ground: Some(CharacterAutostep {}),
        //     ..default()
        // },
        Player,
        ThirdPersonCameraTarget,
    ));
}

fn update_gravity(
    mut gravity_affected: Query<(&mut Transform, &RigidBody), With<Collider>>,
    time: Res<Time>,
) {
    // TODO:
    // This also is pushing below the floor which is not ok.
    // for each item that can have gravity, pull it down to the floor
    let gravity = -1.;
    for mut each in gravity_affected.iter_mut() {
        if each.1 != &RigidBody::Fixed {
            // adding the delta helped, but it needs the flooring to work still.
            each.0.translation.y += gravity * time.delta_seconds();
        }
    }
}

fn check_player_collisions(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut player_query: Query<(&Collider, &mut Transform), With<Player>>,
    object_query: Query<&Collider, Without<Player>>,
) {
    let (player_collider, player_transform) = player_query.single_mut();
    // for object in object_query.iter() {
    match rapier_context.intersection_with_shape(
        player_transform.translation,
        player_transform.rotation,
        player_collider,
        QueryFilter::default(),
    ) {
        Some(entity) => {
            let thing = commands.entity(entity);
        }
        None => {}
    }
    // }
}

fn player_movement_keyboard(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut player_q: Query<&mut Transform, With<Player>>,
    cam_q: Query<&Transform, (With<ThirdPersonCamera>, Without<Player>)>,
) {
    for mut player_transform in player_q.iter_mut() {
        let cam = match cam_q.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap(),
        };

        let mut direction = Vec2::ZERO;

        // forward
        if keys.pressed(KeyCode::W) {
            direction += cam.forward().xz().normalize();
        }

        // back
        if keys.pressed(KeyCode::R) {
            direction += cam.back().xz().normalize();
        }

        // left
        if keys.pressed(KeyCode::A) {
            direction += cam.left().xz().normalize();
        }

        // right
        if keys.pressed(KeyCode::S) {
            direction += cam.right().xz().normalize();
        }

        let movement = direction * time.delta_seconds();
        player_transform.translation.x += movement.x;
        player_transform.translation.z += movement.y;
        let direction: Vec3 = (direction.x, 0.0, direction.y).into();

        // rotate player to face direction he is currently moving
        if direction.length_squared() > 0.0 {
            player_transform.look_to(direction, Vec3::Y);
        }
    }
}
