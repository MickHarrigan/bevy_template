use crate::actions::Actions;
use crate::GameState;
use bevy::prelude::*;
use bevy_rapier3d::{
    prelude::{
        CharacterAutostep, CharacterLength, Collider, KinematicCharacterController, NoUserData,
        QueryFilter, RapierConfiguration, RapierContext, RapierPhysicsPlugin, RigidBody,
    },
    render::RapierDebugRenderPlugin,
};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default())
            .init_resource::<RapierContext>()
            .insert_resource(RapierConfiguration {
                gravity: Vec3::Y * -980.0,
                ..default()
            })
            .add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update,
                (move_player, check_player_collisions, update_gravity)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(
                shape::Capsule {
                    // radius: 0.5,
                    // rings: 0,
                    // depth: 1.0,
                    // latitudes: 16,
                    // longitudes: 32,
                    ..default()
                }
                .into(),
            ),
            material: materials.add(Color::ANTIQUE_WHITE.into()),
            transform: Transform::from_xyz(0., 1., 0.),
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

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut player_controller: Query<&mut KinematicCharacterController>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 10.;
    // super messed up way to make the vec2 work for a xz coordinate
    // but it will be fixed later
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        0.,
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
    );
    for mut player_transform in &mut player_query {
        player_transform.translation += movement;
    }
    for mut player_transform in &mut player_controller {
        player_transform.translation = match player_transform.translation {
            None => Some(movement),
            Some(a) => Some(a + movement),
        };
    }
}
