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
    Buttons(KeyInputCode, KeyInputCode),
    GamepadAxis(GamepadAxisType),
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
            let mut axis_binding: AxisBinding = axis_binding.into();
            match axis_binding {
                AxisBinding::Buttons(neg_key, pos_key) => {
                    self.bound_keys.insert(neg_key);
                    self.bound_keys.insert(pos_key);
                },
                AxisBinding::GamepadAxis(axis) => {
                    // todo: is this needed on main/does the dependency used by bevy fix this?
                    if axis == GamepadAxisType::DPadX {
                        self.bound_keys.insert(GamepadButtonType::DPadLeft.into());
                        self.bound_keys.insert(GamepadButtonType::DPadRight.into());
                        axis_binding = AxisBinding::Buttons(GamepadButtonType::DPadLeft.into(), GamepadButtonType::DPadRight.into());
                    }
                    else if axis == GamepadAxisType::DPadY {
                        self.bound_keys.insert(GamepadButtonType::DPadUp.into());
                        self.bound_keys.insert(GamepadButtonType::DPadDown.into());
                        axis_binding = AxisBinding::Buttons(GamepadButtonType::DPadDown.into(), GamepadButtonType::DPadUp.into());
                    }
                    else {
                        self.bound_axes.insert(axis);
                    }
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
    gamepad_axes_values: HashMap<GamepadAxisType, f32>,
    axes: HashMap<TAxisAction, f32>,
    gamepads: HashSet<Gamepad>,
}

impl<TKeyAction, TAxisAction> Default for ActionInput<TKeyAction, TAxisAction> {
    fn default() -> Self {
        Self { 
            key_states: Default::default(),
            key_actions: Default::default(),
            gamepad_axes_values: Default::default(),
            axes: Default::default(),
            gamepads: Default::default()
        }
    }
}

impl<TKeyAction: ActionMapInput, TAxisAction: ActionMapInput> ActionInput<TKeyAction, TAxisAction>
{
    pub fn get_key_action_state(&self, key: &TKeyAction) -> Option<&KeyState> {
        self.key_actions.get(key)
    }

    pub fn just_pressed(&self, key: TKeyAction) -> bool {
        self.is_key_in_state(key, KeyState::Pressed)
    }

    pub fn held(&self, key: TKeyAction) -> bool {
        self.is_key_in_state(key, KeyState::Held(Default::default()))
    }

    pub fn just_released(&self, key: TKeyAction) -> bool {
        self.is_key_in_state(key, KeyState::Released(Default::default()))
    }

    pub fn used(&self, key: TKeyAction) -> bool {
        self.is_key_in_state(key, KeyState::Used)
    }

    pub fn use_key_action(&mut self, key: TKeyAction) {
        self.key_actions.insert(key.into(), KeyState::Used);
    }

    pub fn get_axis(&self, axis: &TAxisAction) -> f32 {
        if let Some(axis) = self.axes.get(&axis) {
            *axis
        }
        else {
            0.
        }
    }

    pub fn get_xy_axes(&self, x_axis: &TAxisAction, y_axis: &TAxisAction) -> Vec2 {
        Vec2::new(self.get_axis(x_axis), self.get_axis(y_axis))
    }

    fn is_key_in_state(&self, key: TKeyAction, state: KeyState) -> bool {
        if let Some(key_state) = self.key_actions.get(&key.into()) {
            // compare enum variants (not their data)
            std::mem::discriminant(key_state) == std::mem::discriminant(&state)
        }
        else {
            false
        }
    }

    fn key_is_pressed_or_held(&self, key_input_code: &KeyInputCode) -> bool {
        if let Some(Some(KeyState::Pressed | KeyState::Held(..))) = self.key_states.get(key_input_code) {
            true
        }
        else {
            false
        }
    }
}

pub(crate) fn handle_keyboard_button_events<TKeyAction: ActionMapInput + 'static, TAxisAction: ActionMapInput + 'static>(
    mut input: ResMut<ActionInput<TKeyAction, TAxisAction>>,
    kb_input: Res<Input<KeyCode>>,
    map: Res<ActionMap<TKeyAction, TAxisAction>>
) {
    for code in map.bound_keys.iter() {
        if let KeyInputCode::Kb(key) = code {
            input.key_states.insert(*code, get_button_state(&kb_input, key));
        }
    }
}

pub(crate) fn handle_mouse_button_events<TKeyAction: ActionMapInput + 'static, TAxisAction: ActionMapInput + 'static>(
    mut input: ResMut<ActionInput<TKeyAction, TAxisAction>>,
    mouse_input: Res<Input<MouseButton>>,
    map: Res<ActionMap<TKeyAction, TAxisAction>>
) {
    for code in map.bound_keys.iter() {
        if let KeyInputCode::Mouse(button) = code {
            input.key_states.insert(*code, get_button_state(&mouse_input, button));
        }
    }
}

pub(crate) fn handle_gamepad_events<TKeyAction: ActionMapInput + 'static, TAxisAction: ActionMapInput + 'static>(
    mut gamepad_events: EventReader<GamepadEvent>,
    mut input: ResMut<ActionInput<TKeyAction, TAxisAction>>,
    map: Res<ActionMap<TKeyAction, TAxisAction>>,
    gamepad_input: Res<Input<GamepadButton>>,)
{
    for event in gamepad_events.iter() {
        match event {
            GamepadEvent(gamepad, GamepadEventType::Connected) => {
                input.gamepads.insert(*gamepad);
            }
            GamepadEvent(gamepad, GamepadEventType::Disconnected) => {
                input.gamepads.remove(gamepad);
            }
            GamepadEvent(gamepad, GamepadEventType::ButtonChanged(button, _strength)) => {
                let input_code = KeyInputCode::Gamepad(*button);
                if map.bound_keys.get(&input_code).is_some() {
                    input.key_states.insert(input_code, get_button_state(&gamepad_input, &GamepadButton(*gamepad, *button)));
                }
            }
            GamepadEvent(gamepad, GamepadEventType::AxisChanged(axis_type, strength)) => {
                if map.bound_axes.get(axis_type).is_some() {
                    input.gamepad_axes_values.insert(*axis_type, *strength);
                }
            }
        }
    }
}

pub(crate) fn process_button_actions<TKeyAction: ActionMapInput + 'static, TAxisAction: ActionMapInput + 'static>(
    mut input: ResMut<ActionInput<TKeyAction, TAxisAction>>,
    map: Res<ActionMap<TKeyAction, TAxisAction>>
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

pub(crate) fn process_axis_actions<TKeyAction: ActionMapInput + 'static, TAxisAction: ActionMapInput + 'static>(
    mut input: ResMut<ActionInput<TKeyAction, TAxisAction>>,
    map: Res<ActionMap<TKeyAction, TAxisAction>>
) {
    for (axis_action, bindings) in &map.axis_action_bindings {
        let axis_value = bindings.iter().map(|b| {
            let val: f32 = match b {
                AxisBinding::Buttons(neg, pos) => {
                    let mut val = 0.;
                    if input.key_is_pressed_or_held(neg) {
                        val -= 1.;
                    }
                    if input.key_is_pressed_or_held(pos) {
                        val += 1.;
                    }

                    val
                },
                AxisBinding::GamepadAxis(gamepad_axis) => {
                    *input.gamepad_axes_values.get(gamepad_axis).unwrap_or(&0.)
                },
            };

            // todo: deadzone conf:
            // todo: normalize the value given the deadzone (0.0..1.0)
            if val.abs() > 0.2 { val } else { 0. }
        }).fold(0., |a: f32, b: f32| if a.abs() > b.abs() { a } else { b });

        input.axes.insert(*axis_action, axis_value);
    }
}

fn get_button_state<T: Copy + Eq + Hash>(
    input: &Input<T>,
    button: &T
) -> Option<KeyState> {
    if input.just_pressed(*button) {
        Some(KeyState::Pressed)
    }
    else if input.just_released(*button) {
        Some(KeyState::Released(ActiveKeyData {
            // todo: actual dur
            duration: 0.
        }))
    }
    else if input.pressed(*button) {
        Some(KeyState::Held(ActiveKeyData {
            // todo: actual dur
            duration: 0.
        }))
    }
    else {
        None
    }
}