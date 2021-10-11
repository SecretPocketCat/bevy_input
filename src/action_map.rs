use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};
use maplit::{hashset};
use bevy::{input::{gamepad::{GamepadAxisType, GamepadEvent, GamepadEventType}, ElementState, keyboard::KeyboardInput}, prelude::*};

trait ActionMapInput : Hash + Eq + Clone + Send + Sync {}

#[derive(Clone, Debug, PartialEq)]
pub enum AxisInput {
    Kb(KeyCode, KeyCode),
    Gamepad(GamepadAxisType)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Axis {
    deadzone: f32,
    value: AxisInput,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeyInputCode {
    Kb(KeyCode),
    Gamepad(GamepadButtonType),
    Mouse(u8),
}

#[derive(Clone, Debug, PartialEq)]
enum Binding {
    Keys(HashSet<KeyInputCode>),
    Axis(Axis),
}

// macro_rules! impl_from_key_input {
//     ($key: ty, $enum: expr) => {
//         impl From<$key> for Binding {
//             fn from(keycode: $key) -> Self {
//                 $enum(hashset![keycode])
//             }
//         }
        
//         impl From<Vec<$key>> for Binding {
//             fn from(keys: Vec<$key>) -> Self {
//                 $enum(keys.into_iter().collect())
//             }
//         }
//     };
// }

// impl_from_key_input!(KeyCode, Binding::Keys);
// impl_from_key_input!(GamepadButtonType, Binding::GamePadButtons);
// impl_from_key_input!(Axis<AxisInput>, Binding::Axis);
// impl_from_key_input!(Axis<XyAxes>, Binding::XyAxes);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ActionBindings {
    bindings: Vec<Binding>,
}

#[derive(Debug, PartialEq)]
pub enum KeyState {
    Pressed,
    Held(ActiveKeyData),
    Released(ActiveKeyData),
    Used,
}

#[derive(Default, Debug, PartialEq)]
struct ActiveKeyData {
    duration: f32,
}

#[derive(Default)]
pub struct ActionMap<TKeyAction, TAxisAction> {
    key_actions: HashMap<TKeyAction, ActionBindings>,
    axis_actions: HashMap<TAxisAction, ActionBindings>,
    // todo: redo to bound keys...???
    bound_keys: HashMap<KeyInputCode, Vec<(TKeyAction, ActionBindings)>>,
    bound_gamepad_buttons: HashMap<GamepadButtonType, Vec<ActionBindings>>,
}

impl<TKeyAction: ActionMapInput, TAxisAction: ActionMapInput> ActionMap<TKeyAction, TAxisAction>
{
    // todo: bind should validate actions don't overlap & return result
    pub fn bind_key_action<K: Into<TKeyAction>, B: Into<Binding>>(&mut self, action: K, binding: B) -> &mut Self {
        let key = action.into();
        if !self.key_actions.contains_key(&key) {
            self.key_actions.insert(key, Default::default());
        }

        if let Some(action) = self.key_actions.get_mut(&key) {
            let binding: Binding = binding.into();

            match binding {
                Binding::Keys(kb_keys) => {
                    self.bound_keys.extend(kb_keys);
                },
                Binding::GamePadButtons(gp_btns) => {
                    self.bound_gamepad_buttons.extend(gp_btns);
                },
                Binding::Axis(Axis { value: AxisInput::Kb(axis_key_neg, axis_key_pos), .. }) => {
                    self.bound_keys.insert(axis_key_neg);
                    self.bound_keys.insert(axis_key_pos);
                },
                _ => {}
            }

            action.bindings.push(binding);
        }

   

        self
    }

    // todo: bind should validate actions don't overlap & return result
    pub fn bind_axis<A: Into<TAxisAction>, B: Into<Binding>>(&mut self, action: A, axis_binding: B) -> &mut Self {
        let key = action.into();
        if !self.axis_actions.contains_key(&key) {
            self.axis_actions.insert(key, Default::default());
        }

        if let Some(actions) = self.axis_actions.get_mut(&key) {
            actions.bindings.push(axis_binding.into());
        }
        self
    }
}

#[derive(Default)]
pub struct ActionInput<TKeyAction, TAxisAction> {
    pub(crate) key_statuses: HashMap<KeyInputCode, Option<KeyState>>,
    key_actions: HashMap<TKeyAction, KeyState>,
    axes: HashMap<TAxisAction, f32>,
    gamepads: HashSet<Gamepad>,
}

impl<TKey: ActionMapInput, TAxis: ActionMapInput> ActionInput<TKey, TAxis>
{
    pub fn get_key_action_state(&self, key: &TKey) -> Option<&KeyState> {
        self.key_actions.get(key)
    }

    pub fn just_pressed(&self, key: TKey) -> bool {
        self.is_key_in_state(key, KeyState::Pressed)
    }

    pub fn held(&self, key: TKey) -> bool {
        self.is_key_in_state(key, KeyState::Held(Default::default()))
    }

    pub fn just_released(&self, key: TKey) -> bool {
        self.is_key_in_state(key, KeyState::Released(Default::default()))
    }

    pub fn used(&self, key: TKey) -> bool {
        self.is_key_in_state(key, KeyState::Used)
    }

    pub fn use_key_action(&self, key: TKey) {
        self.key_actions.insert(key.into(), KeyState::Used);
    }

    pub fn get_axis(&self, axis: &TAxis) -> f32 {
        if let Some(axis) = self.axes.get(&axis) {
            *axis
        }
        else {
            0.
        }
    }

    pub fn get_xy_axes(&self, x_axis: &TAxis, y_axis: &TAxis) -> Vec2 {
        Vec2::new(self.get_axis(x_axis), self.get_axis(y_axis))
    }

    fn is_key_in_state(&self, key: TKey, state: KeyState) -> bool {
        if let Some(key_state) = self.key_actions.get(&key.into()) {
            // compare enum variants (not their data)
            std::mem::discriminant(key_state) == std::mem::discriminant(&state)
        }
        else {
            false
        }
    }
}

// this should run after bevy input
fn handle_key_events<TKey: ActionMapInput + 'static, TAxis: ActionMapInput + 'static>(
    mut input: ResMut<ActionInput<TKey, TAxis>>,
    kb_input: Res<Input<KeyCode>>,
    map: Mut<ActionMap<TKey, TAxis>>
) {
    for code in map.bound_keys.keys() {
        if let KeyInputCode::Kb(key) = code {
            let mut state;
        
            if kb_input.just_pressed(*key) {
                state = Some(KeyState::Pressed);
            }
            else if kb_input.just_released(*key) {
                state = Some(KeyState::Released(ActiveKeyData {
                    // todo: actual dur
                    duration: 0.
                }));
            }
            else if kb_input.pressed(*key) {
                state = Some(KeyState::Held(ActiveKeyData {
                    // todo: actual dur
                    duration: 0.
                }));
            }
            else {
                state = None;
            }
    
            input.key_statuses.insert(*code, state);
        }
    }
}

fn process_key_actions<TKey: ActionMapInput + 'static, TAxis: ActionMapInput + 'static>(
    mut input: ResMut<ActionInput<TKey, TAxis>>,
    map: Mut<ActionMap<TKey, TAxis>>
) {
    // todo: process inputs into actions (and send action events)
    // this should probably be moved to its own system to handle mixed kb/mouse/gamepad

    for (action_key, action) in map.key_actions {
        let current_state = input.get_key_action_state(&action_key);
        match current_state {
            None | Some(KeyState::Released(..) | KeyState::Used) => {
                action.bindings.iter().find(|binding| {
                    binding.
                });
                // look for initial press (from any of the keys?)
            },
            Some(KeyState::Pressed | KeyState::Held(..)) => {
                // check if still pressed
            },
        }
    }
}

// fn handle_gamepad_events<TKey: ActionMapInput, TAxis: ActionMapInput>(mut gamepad_events: EventReader<GamepadEvent>, mut input: ResMut<ActionInput<TKey, TAxis>>)
// where
//     TKey: 'static + Debug,
// {
//     for event in gamepad_events.iter() {
//         match event {
//             GamepadEvent(gamepad, GamepadEventType::Connected) => {
//                 input.gamepads.insert(*gamepad);
//             }
//             GamepadEvent(gamepad, GamepadEventType::Disconnected) => {
//                 input.gamepads.remove(gamepad);
//             }
//             GamepadEvent(_, GamepadEventType::ButtonChanged(button, strength)) => {
//                 if *strength > 0. {
//                     input.pressed_buttons.insert(*button, *strength);
//                 } else {
//                     input.pressed_buttons.remove(button);
//                 }
//             }
//             GamepadEvent(_, GamepadEventType::AxisChanged(axis_type, strength)) => {
//                 // todo: handle
//             }
//         }
//     }
// }