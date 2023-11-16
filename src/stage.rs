use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::GameState;

pub struct StagePlugin;

impl Plugin for StagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), build_stage);
    }
}

fn build_stage(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ground
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(200.0).into()),
            material: materials.add(Color::GREEN.into()),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Collider::cuboid(100., 0., 100.),
    ));

    // box
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Cube::new(1.0).into()),
            material: materials.add(Color::BLUE.into()),
            transform: Transform::from_xyz(2., 0.5, -2.),
            ..default()
        },
        Collider::cuboid(0.5, 0.5, 0.5),
        RigidBody::Fixed,
    ));

    // edges of play field
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(0.5).into()),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(0., 0.25, -100.)
                .with_scale(Vec3::new(400., 1., 1.))
                .with_rotation(Quat::from_rotation_x(PI / 2.)),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(50., 0., 0.25),
    ));
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(0.5).into()),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(0., 0.25, 100.)
                .with_scale(Vec3::new(400., 1., 1.))
                .with_rotation(Quat::from_rotation_x(-PI / 2.)),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(50., 0., 0.25),
    ));
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(0.5).into()),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(-100., 0.25, 0.)
                .with_scale(Vec3::new(1., 1., 400.))
                .with_rotation(Quat::from_rotation_z(-PI / 2.)),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(50., 0., 0.25),
    ));
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(0.5).into()),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(100., 0.25, 0.)
                .with_scale(Vec3::new(1., 1., 400.))
                .with_rotation(Quat::from_rotation_z(PI / 2.)),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(50., 0., 0.25),
    ));
}

fn change_stage() {}
