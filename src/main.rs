#![feature(destructuring_assignment)]
#![feature(if_let_guard)]

mod input;
mod action_map;

use bevy::prelude::*;
// use input::InputPlugin;

// #[derive(Default)]
// pub struct GameInput {
//     pub movement: Vec2,
//     pub aim: Vec2,
//     pub jump: Option<KeyState>,
//     pub last_time_jump_pressed: f32,
//     pub whack: Option<KeyState>,
//     pub reset: Option<KeyState>,
//     pub debug: Option<KeyState>,
// }

// const JUMP_KEYS: [Key; 6] = [
//     Key::GamePad(GamepadButtonType::South),
//     Key::GamePad(GamepadButtonType::LeftTrigger),
//     Key::GamePad(GamepadButtonType::RightTrigger),
//     Key::Kb(KeyCode::Space),
//     Key::Kb(KeyCode::W),
//     Key::Kb(KeyCode::Up),
// ];
// const WHACK_KEYS: [Key; 8] = [
//     Key::GamePad(GamepadButtonType::East),
//     Key::GamePad(GamepadButtonType::West),
//     Key::GamePad(GamepadButtonType::North),
//     Key::GamePad(GamepadButtonType::LeftTrigger2),
//     Key::GamePad(GamepadButtonType::RightTrigger2),
//     Key::Kb(KeyCode::LShift),
//     Key::Kb(KeyCode::J),
//     Key::Kb(KeyCode::Numpad0),
// ];
// const RESET_KEYS: [Key; 2] = [Key::GamePad(GamepadButtonType::Select), Key::Kb(KeyCode::R)];
// const DEBUG_KEYS: [Key; 1] = [Key::Kb(KeyCode::F1)];

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugin(InputPlugin)
        .run();
}
