#![feature(destructuring_assignment)]
#![feature(if_let_guard)]

mod plugin;
mod action_map;

use action_map::{ActionInput, ActionMap, ActionMapInput};
use bevy::prelude::*;
use plugin::ActionInputPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum InputAction {
    Jump,
    Shoot,
}

impl ActionMapInput for InputAction {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum InputAxis {

}

impl ActionMapInput for InputAxis {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ActionInputPlugin::<InputAction, InputAxis>::new())
        .add_startup_system(setup)
        .add_system(print_actions)
        .run();
}

fn setup(mut map: ResMut<ActionMap<InputAction, InputAxis>>) {
    map
        .bind_key_action(InputAction::Jump, vec![KeyCode::Space.into()])
        .bind_key_action(InputAction::Jump, vec![KeyCode::W.into()])
        .bind_key_action(InputAction::Shoot, vec![KeyCode::LShift.into()])
        .bind_key_action(InputAction::Shoot, vec![KeyCode::Numpad0.into(), KeyCode::RControl.into()]);
}

fn print_actions(input: Res<ActionInput<InputAction, InputAxis>>) {
    println!("{:?} => {:?}", InputAction::Jump, input.get_key_action_state(&InputAction::Jump));
    println!("{:?} => {:?}", InputAction::Shoot, input.get_key_action_state(&InputAction::Shoot));
}
