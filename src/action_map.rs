use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};
use maplit::{hashset};
use bevy::{input::{gamepad::{GamepadAxisType, GamepadEvent, GamepadEventType}, ElementState, keyboard::KeyboardInput}, prelude::*};

// todo: replace by a trait alias?
pub trait ActionMapInput : Debug + Hash + Eq + Clone + Copy + Send + Sync {}

type KeyActionBinding = HashSet<KeyInputCode>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AxisBinding {
    Kb(KeyCode, KeyCode),
    Gamepad(GamepadAxisType)
}

// // todo: need to handle this not eq/hash trait impl for f32  - move deadzone somplace else?
// #[derive(Clone, Debug, PartialEq, Eq, Hash)]
// pub struct AxisBinding {
//     deadzone: f32,
//     input: AxisBinding,
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyInputCode {
    Kb(KeyCode),
    Gamepad(GamepadButtonType),
    Mouse(MouseButton),
}

impl KeyInputCode {
    fn keyboard_button(kb_button: KeyCode) -> Self {
        Self::Kb(kb_button)
    }

    fn gamepad_button(gamepad_button: GamepadButtonType) -> Self {
        Self::Gamepad(gamepad_button)
    }

    fn mouse_button(mouse_button: MouseButton) -> Self {
        Self::Mouse(mouse_button)
    }
}

impl From<KeyCode> for KeyInputCode {
    fn from(kb_button: KeyCode) -> Self {
        Self::keyboard_button(kb_button)
    }
}

impl From<GamepadButtonType> for KeyInputCode {
    fn from(gamepad_button: GamepadButtonType) -> Self {
        Self::gamepad_button(gamepad_button)
    }
}

impl From<MouseButton> for KeyInputCode {
    fn from(mouse_button: MouseButton) -> Self {
        Self::mouse_button(mouse_button)
    }
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

#[derive(Debug, PartialEq)]
pub enum KeyState {
    Pressed,
    Held(ActiveKeyData),
    Released(ActiveKeyData),
    Used,
}

#[derive(Default, Debug, PartialEq)]
pub struct ActiveKeyData {
    duration: f32,
}

pub struct ActionMap<TKeyAction, TAxisAction> {
    key_action_bindings: HashMap<TKeyAction, Vec<KeyActionBinding>>,
    axis_action_bindings: HashMap<TAxisAction, HashSet<AxisBinding>>,
    bound_keys: HashSet<KeyInputCode>,
    bound_axes: HashSet<GamepadAxisType>,
}

impl<TKeyAction, TAxisAction> Default for ActionMap<TKeyAction, TAxisAction> {
    fn default() -> Self {
        Self {
            key_action_bindings: Default::default(),
            axis_action_bindings: Default::default(),
            bound_keys: Default::default(),
            bound_axes: Default::default() }
    }
}

impl<TKeyAction: ActionMapInput, TAxisAction: ActionMapInput> ActionMap<TKeyAction, TAxisAction>
{
    // todo: bind should validate actions don't overlap & return result
    pub fn bind_key_action<K: Into<TKeyAction>, B: IntoIterator<Item = KeyInputCode>>(&mut self, action: K, binding: B) -> &mut Self {
        let key = action.into();
        let binding: KeyActionBinding = binding.into_iter().collect();
        if !self.key_action_bindings.contains_key(&key) {
            self.key_action_bindings.insert(key, Default::default());
        }

        if let Some(action) = self.key_action_bindings.get_mut(&key) {
            let binding: KeyActionBinding = binding.into();
            self.bound_keys.extend(binding.clone());
            action.push(binding);
        }

        self
    }

    // todo: bind should validate actions don't overlap & return result?
    // does that actually apply to axes?
    pub fn bind_axis<A: Into<TAxisAction>, B: Into<AxisBinding>>(&mut self, action: A, axis_binding: B) -> &mut Self {
        let key = action.into();
        if !self.axis_action_bindings.contains_key(&key) {
            self.axis_action_bindings.insert(key, Default::default());
        }

        if let Some(action) = self.axis_action_bindings.get_mut(&key) {
            let axis_binding: AxisBinding = axis_binding.into();
            match axis_binding {
                AxisBinding::Kb(neg_key, pos_key) => {
                    self.bound_keys.insert(KeyInputCode::Kb(neg_key));
                    self.bound_keys.insert(KeyInputCode::Kb(pos_key));
                },
                AxisBinding::Gamepad(axis) => {
                    self.bound_axes.insert(axis);
                },
            }

            action.insert(axis_binding);
        }
        self
    }
}

pub struct ActionInput<TKeyAction, TAxisAction> {
    pub(crate) key_states: HashMap<KeyInputCode, Option<KeyState>>,
    key_actions: HashMap<TKeyAction, KeyState>,
    axes: HashMap<TAxisAction, f32>,
    gamepads: HashSet<Gamepad>,
}

impl<TKeyAction, TAxisAction> Default for ActionInput<TKeyAction, TAxisAction> {
    fn default() -> Self {
        Self { 
            key_states: Default::default(),
            key_actions: Default::default(),
            axes: Default::default(),
            gamepads: Default::default()
        }
    }
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

    pub fn use_key_action(&mut self, key: TKey) {
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

pub(crate) fn handle_keyboard_button_events<TKey: ActionMapInput + 'static, TAxis: ActionMapInput + 'static>(
    mut input: ResMut<ActionInput<TKey, TAxis>>,
    kb_input: Res<Input<KeyCode>>,
    map: Res<ActionMap<TKey, TAxis>>
) {
    for code in map.bound_keys.iter() {
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
    
            input.key_states.insert(*code, state);
        }
    }
}

pub(crate) fn handle_mouse_button_events<TKey: ActionMapInput + 'static, TAxis: ActionMapInput + 'static>(
    mut input: ResMut<ActionInput<TKey, TAxis>>,
    mouse_input: Res<Input<MouseButton>>,
    map: Res<ActionMap<TKey, TAxis>>
) {
    for code in map.bound_keys.iter() {
        if let KeyInputCode::Mouse(key) = code {
            let mut state;
        
            if mouse_input.just_pressed(*key) {
                state = Some(KeyState::Pressed);
            }
            else if mouse_input.just_released(*key) {
                state = Some(KeyState::Released(ActiveKeyData {
                    // todo: actual dur
                    duration: 0.
                }));
            }
            else if mouse_input.pressed(*key) {
                state = Some(KeyState::Held(ActiveKeyData {
                    // todo: actual dur
                    duration: 0.
                }));
            }
            else {
                state = None;
            }
    
            input.key_states.insert(*code, state);
        }
    }
}

pub(crate) fn process_key_actions<TKey: ActionMapInput + 'static, TAxis: ActionMapInput + 'static>(
    mut input: ResMut<ActionInput<TKey, TAxis>>,
    map: Res<ActionMap<TKey, TAxis>>
) {
    'actions: for (action_key, action) in map.key_action_bindings.iter() {
        let current_state = input.get_key_action_state(&action_key);
        match current_state {
            None | Some(KeyState::Released(..) | KeyState::Used) => {
                'bindings: for binding_keys in action {
                    let mut just_pressed_at_least_one_key = false;

                    for k in binding_keys.iter() {
                        if let Some(key_state) = input.key_states.get(k) {
                            match key_state {
                                Some(KeyState::Pressed) => {
                                    just_pressed_at_least_one_key = true;
                                    continue;
                                },
                                Some(KeyState::Held(..)) => {
                                    continue;
                                },
                                _ => { 
                                    continue 'bindings
                                },
                            }
                        }
                        else {
                            continue 'bindings
                        }
                    }

                    // at least one 1 key was just pressed, the rest can be held
                    if just_pressed_at_least_one_key {
                        input.key_actions.insert(*action_key, KeyState::Pressed);
                        continue 'actions;
                    }
                }

                input.key_actions.remove(&action_key);
            },
            Some(KeyState::Pressed | KeyState::Held(..)) => {
                // check if all keys are still held
                'held_bindings: for binding_keys in action {
                    for k in binding_keys.iter() {
                        if let Some(key_state) = input.key_states.get(k) {
                            match key_state {
                                Some(KeyState::Pressed | KeyState::Held(..)) => {
                                    continue;
                                },
                                _ => { 
                                    continue 'held_bindings
                                },
                            }
                        }
                        else {
                            continue 'held_bindings
                        }
                    }

                    input.key_actions.insert(*action_key, KeyState::Held(ActiveKeyData {
                        duration: 0. // todo: actual duration
                    }));
                    continue 'actions;
                }

                input.key_actions.insert(*action_key, KeyState::Released(ActiveKeyData {
                    duration: 0. // todo: actual duration
                }));
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