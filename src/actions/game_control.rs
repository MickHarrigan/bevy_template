use bevy::prelude::{Input, KeyCode, Res};

pub enum GameControl {
    Up,
    Down,
    Left,
    Right,
}

impl GameControl {
    pub fn pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Up => keyboard_input.pressed(KeyCode::W),
            GameControl::Down => keyboard_input.pressed(KeyCode::R),
            GameControl::Left => keyboard_input.pressed(KeyCode::A),
            GameControl::Right => keyboard_input.pressed(KeyCode::S),
        }
    }
}

pub fn get_movement(control: GameControl, input: &Res<Input<KeyCode>>) -> f32 {
    if control.pressed(input) {
        1.0
    } else {
        0.0
    }
}
