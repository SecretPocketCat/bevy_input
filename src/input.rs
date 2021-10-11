// use bevy::{
//     input::{
//         gamepad::{GamepadAxisType, GamepadEvent, GamepadEventType},
//         InputSystem,
//     },
//     prelude::*,
// };


// // todo: 
// // get mapping from a resource - json? - different json loading mod?
// // fire events based on the bound actions
// //  - btn
// //  - axis
// //  - mouse pos (grab the existing variant, but keep that as a different module?)
// // map the events outside the plugin (in a game)

// // todo: instead of polling every step just use the gamepad & KB actions

// // allow both polling and events for actions 

// pub struct InputPlugin;
// impl Plugin for InputPlugin {
//     fn build(&self, app: &mut App) {
//         app.init_resource::<ActionMap>()
//             .add_system(handle_gamepad_events);
//     }
// }

// pub struct ActionMap {
    
// }

// impl Default for ActionMap {
//     fn default() -> Self {
//         Self {  }
//     }
// }

// pub enum ActionInput {
//     Button(Key),
//     Axis(Axis)
// }

// pub struct Action {
//     input: ActionInput,
//     enabled: bool,
// }

// pub enum Axis {
//     Kb(KeyCode, KeyCode),
//     Gamepad(GamepadAxisType)
// }

// pub enum Key {
//     Kb(KeyCode),
//     GamePad(GamepadButtonType),
// }

// impl From<KeyCode> for Key {
//     fn from(code: KeyCode) -> Self {
//         Key::Kb(code)
//     }
// }

// impl From<GamepadButtonType> for Key {
//     fn from(code: GamepadButtonType) -> Self {
//         Key::GamePad(code)
//     }
// }

// #[derive(Debug, PartialEq)]
// pub enum KeyState {
//     Pressed,
//     Held { duration: f32 },
//     Released { duration: f32 },
//     Used,
// }

// struct MyGamepad(Gamepad);

// fn handle_gamepad_events(
//     mut commands: Commands,
//     gamepad: Option<Res<MyGamepad>>,
//     mut gamepad_evr: EventReader<GamepadEvent>,
// ) {
//     for GamepadEvent(id, kind) in gamepad_evr.iter() {
//         match kind {
//             GamepadEventType::Connected => {
//                 // if we don't have any gamepad yet, use this one
//                 if gamepad.is_none() {
//                     commands.insert_resource(MyGamepad(*id));
//                 }
//             }
//             GamepadEventType::Disconnected => {
//                 // if it's the one we previously associated with the player,
//                 // disassociate it:
//                 if let Some(MyGamepad(old_id)) = gamepad.as_deref() {
//                     if old_id == id {
//                         commands.remove_resource::<MyGamepad>();
//                     }
//                 }
//             }
//             // other events are irrelevant
//             _ => {}
//         }
//     }
// }

// fn game_input(
//     mut input: ResMut<GameInput>,
//     keyboard_input: Res<Input<KeyCode>>,
//     pad_input: Res<Input<GamepadButton>>,
//     axes: Res<Axis<GamepadAxis>>,
//     gamepad: Option<Res<MyGamepad>>,
//     time: ScaledTime,
// ) {
//     input.jump = get_key_state(
//         &keyboard_input,
//         &pad_input,
//         &gamepad,
//         &time,
//         &input.jump,
//         &JUMP_KEYS,
//     );
//     if let Some(KeyState::Pressed) = input.jump {
//         input.last_time_jump_pressed = time.time.seconds_since_startup() as f32;
//     }

//     input.whack = get_key_state(
//         &keyboard_input,
//         &pad_input,
//         &gamepad,
//         &time,
//         &input.jump,
//         &WHACK_KEYS,
//     );

//     input.reset = get_key_state(
//         &keyboard_input,
//         &pad_input,
//         &gamepad,
//         &time,
//         &input.reset,
//         &RESET_KEYS,
//     );

//     input.debug = get_key_state(
//         &keyboard_input,
//         &pad_input,
//         &gamepad,
//         &time,
//         &input.reset,
//         &DEBUG_KEYS,
//     );

//     // todo: fn
//     let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
//     let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);
//     let down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
//     let up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
//     let kb_x = (-(left as i8) + right as i8) as f32;
//     let kb_y = (-(down as i8) + up as i8) as f32;
//     input.movement = Vec2::new(kb_x, kb_y);

//     if input.movement == Vec2::ZERO {
//         // try gamepad stick
//         if let Some(pad) = gamepad {
//             input.movement = get_pad_axis(
//                 GamepadAxisType::LeftStickX,
//                 GamepadAxisType::LeftStickY,
//                 pad.0,
//                 &axes);

//             // try gamped dpad
//             if input.movement == Vec2::ZERO {
//                 let d_left = GamepadButton(pad.0, GamepadButtonType::DPadLeft);
//                 let d_right = GamepadButton(pad.0, GamepadButtonType::DPadRight);
//                 let d_up = GamepadButton(pad.0, GamepadButtonType::DPadDown);
//                 let d_down = GamepadButton(pad.0, GamepadButtonType::DPadUp);

//                 let left = pad_input.pressed(d_left);
//                 let right = pad_input.pressed(d_right);
//                 let up = pad_input.pressed(d_up);
//                 let down = pad_input.pressed(d_down);
//                 let dpad_x = (-(left as i8) + right as i8) as f32;
//                 let dpad_y = (-(down as i8) + up as i8) as f32;
//                 input.movement = Vec2::new(dpad_x, dpad_y);
//             }

//             input.aim = get_pad_axis(
//                 GamepadAxisType::RightStickX,
//                 GamepadAxisType::RightStickY,
//                 pad.0,
//                 &axes);
//         }
//     }
// }

// fn get_pad_axis(axis_x: GamepadAxisType, axis_y: GamepadAxisType, pad: Gamepad, axes: &Axis<GamepadAxis>,) -> Vec2 {
//     let x_stick = GamepadAxis(pad, axis_x);
//     let y_stick = GamepadAxis(pad, axis_y);
//     if let (Some(x), Some(y)) = (axes.get(x_stick), axes.get(y_stick)) {
//         let dir = Vec2::new(x, y);
//         if dir.length() > 0.1 {
//             // deadzone
//             Vec2::new(x, y)
//         }
//         else {
//             Vec2::ZERO
//         }
//     }
//     else {
//         Vec2::ZERO
//     }
// }

// fn get_key_state(
//     keyboard_input: &Res<Input<KeyCode>>,
//     pad_input: &Res<Input<GamepadButton>>,
//     gamepad: &Option<Res<MyGamepad>>,
//     time: &ScaledTime,
//     prev_state: &Option<KeyState>,
//     keys: &[Key],
// ) -> Option<KeyState> {
//     for key in keys.iter() {
//         let just_released = get_released(keyboard_input, pad_input, gamepad, key);
//         if let Some(KeyState::Used) = prev_state {
//             if just_released {
//                 return None;
//             } else {
//                 return Some(KeyState::Used);
//             }
//         } else if get_pressed(keyboard_input, pad_input, gamepad, key) {
//             return Some(KeyState::Pressed);
//         } else if get_held(keyboard_input, pad_input, gamepad, key) {
//             return match prev_state {
//                 Some(state) if let KeyState::Held { duration } = state => Some(KeyState::Held { duration: duration + time.delta_seconds() }),
//                 _ => Some(KeyState::Held { duration: 0. })
//             };
//         } else if just_released {
//             return match prev_state {
//                 Some(state) if let KeyState::Held { duration } = state => Some(KeyState::Released { duration: duration + time.delta_seconds() }),
//                 _ => Some(KeyState::Released { duration: 0. })
//             };
//         }
//     }

//     None
// }

// // todo: better generic impl for the whole input...
// fn get_pressed(
//     keyboard_input: &Input<KeyCode>,
//     pad_input: &Input<GamepadButton>,
//     gamepad: &Option<Res<MyGamepad>>,
//     key: &Key,
// ) -> bool {
//     match key {
//         Key::Kb(code) => keyboard_input.just_pressed(*code),
//         Key::GamePad(code) => {
//             if let Some(pad) = gamepad {
//                 let pad_btn = GamepadButton(pad.0, *code);
//                 pad_input.just_pressed(pad_btn)
//             } else {
//                 false
//             }
//         }
//     }
// }

// fn get_held(
//     keyboard_input: &Input<KeyCode>,
//     pad_input: &Input<GamepadButton>,
//     gamepad: &Option<Res<MyGamepad>>,
//     key: &Key,
// ) -> bool {
//     match key {
//         Key::Kb(code) => keyboard_input.pressed(*code),
//         Key::GamePad(code) => {
//             if let Some(pad) = gamepad {
//                 let pad_btn = GamepadButton(pad.0, *code);
//                 pad_input.pressed(pad_btn)
//             } else {
//                 false
//             }
//         }
//     }
// }

// fn get_released(
//     keyboard_input: &Input<KeyCode>,
//     pad_input: &Input<GamepadButton>,
//     gamepad: &Option<Res<MyGamepad>>,
//     key: &Key,
// ) -> bool {
//     match key {
//         Key::Kb(code) => keyboard_input.just_released(*code),
//         Key::GamePad(code) => {
//             if let Some(pad) = gamepad {
//                 let pad_btn = GamepadButton(pad.0, *code);
//                 pad_input.just_released(pad_btn)
//             } else {
//                 false
//             }
//         }
//     }
// }
