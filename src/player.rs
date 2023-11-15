use crate::actions::Actions;
use crate::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Capsule { ..default() }.into()),
            material: materials.add(Color::ANTIQUE_WHITE.into()),
            transform: Transform::from_xyz(0., 1., 0.),
            ..Default::default()
        })
        .insert(Player);
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
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
}
