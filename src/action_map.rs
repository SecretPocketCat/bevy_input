use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};
use bevy::{input::{gamepad::{GamepadAxisType, GamepadEvent, GamepadEventType}, ElementState, keyboard::KeyboardInput}, prelude::*};
use itertools::Itertools;

use crate::inputs_vec;

const DEADZONE_PRECISION: f32 = 10000.;

pub trait ActionMapInput = Debug + Hash + Eq + Clone + Copy + Send + Sync;

pub(crate) type KeyActionBinding = HashSet<ButtonCode>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum AxisBinding {
    Buttons(ButtonCode, ButtonCode),
    GamepadAxis(GamepadAxisType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum ButtonCode {
    Kb(KeyCode),
    Gamepad(GamepadButtonType),
    Mouse(MouseButton),
}

impl ButtonCode {
    fn keyboard_button(kb_button: KeyCode) -> Self {
        Self::Kb(kb_button)
    }

    fn gamepad_button(gamepad_button: GamepadButtonType) -> Self {
        Self::Gamepad(gamepad_button)
    }

    fn mouse_button(mouse_button: MouseButton) -> Self {
        Self::Mouse(mouse_button)
    }

    fn player_data(&self, id: Option<usize>) -> PlayerData<Self> {
        PlayerData::<Self> {
            value: *self,
            id
        }
    }
}

// todo: reduce duplication by using a macro
impl From<KeyCode> for ButtonCode {
    fn from(kb_button: KeyCode) -> Self {
        Self::keyboard_button(kb_button)
    }
}

impl From<GamepadButtonType> for ButtonCode {
    fn from(gamepad_button: GamepadButtonType) -> Self {
        Self::gamepad_button(gamepad_button)
    }
}

impl From<MouseButton> for ButtonCode {
    fn from(mouse_button: MouseButton) -> Self {
        Self::mouse_button(mouse_button)
    }
}

#[derive(Debug, PartialEq)]
pub enum ButtonState {
    Pressed,
    Held,
    Released,
}

#[derive(Debug, PartialEq)]
pub enum ActionState {
    Pressed,
    Held(ActiveKeyData),
    Released(ActiveKeyData),
    Used,
}

impl ActionState {
    pub fn duration(&self) -> f32 {
        match self {
            ActionState::Pressed => 0.,
            ActionState::Held(data) => data.duration,
            ActionState::Released(data) => data.duration,
            ActionState::Used => 0.,
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct ActiveKeyData {
    duration: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct PlayerData<T> {
    pub(crate) id: Option<usize>,
    pub(crate) value: T,
}

type DeviceData<T> = PlayerData<T>;

impl<T> PlayerData<T> {
    pub fn new(action: T) -> Self {
        Self {
            value: action,
            id: None
        }
    }

    pub fn new_with_id(action: T, id: usize) -> Self {
        Self {
            value: action,
            id: Some(id)
        }
    }
}

impl<T> From<T> for PlayerData<T> {
    fn from(data: T) -> Self {
        Self::new(data)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct ActionMap<TKeyAction: ActionMapInput, TAxisAction: ActionMapInput> {
    key_action_bindings: HashMap<PlayerData<TKeyAction>, Vec<KeyActionBinding>>,
    axis_action_bindings: HashMap<PlayerData<TAxisAction>, HashSet<(AxisBinding, u32)>>,
    #[cfg_attr(feature = "serialize", serde(skip))]
    bound_keys: HashSet<PlayerData<ButtonCode>>,
    #[cfg(feature = "validation")]
    #[cfg_attr(feature = "serialize", serde(skip))]
    pub(crate) bound_key_combinations: HashSet<PlayerData<Vec<ButtonCode>>>,
    #[cfg(feature = "validation")]
    #[cfg_attr(feature = "serialize", serde(skip))]
    pub(crate) potential_bound_key_combinations: HashSet<PlayerData<Vec<ButtonCode>>>,
    #[cfg_attr(feature = "serialize", serde(skip))]
    bound_axes: HashSet<GamepadAxisType>,
}

impl<TKeyAction: ActionMapInput, TAxisAction: ActionMapInput> Default for ActionMap<TKeyAction, TAxisAction> {
    fn default() -> Self {
        Self {
            key_action_bindings: Default::default(),
            axis_action_bindings: Default::default(),
            bound_keys: Default::default(),
            bound_key_combinations: Default::default(),
            potential_bound_key_combinations: Default::default(),
            bound_axes: Default::default(),
        }
    }
}

#[cfg(not(feature = "multiplayer"))]
impl<TKeyAction: ActionMapInput, TAxisAction: ActionMapInput> ActionMap<TKeyAction, TAxisAction>
{
    pub fn bind_button_action<K: Into<TKeyAction>, B: Into<ButtonCode>>(&mut self, action: K, button: B) -> &mut Self {
        self.bind_button_action_internal(action, button, None)
    }

    pub fn bind_button_combination_action<K: Into<TKeyAction>, B: IntoIterator<Item = ButtonCode>>(&mut self, action: K, binding: B) -> &mut Self {
        self.bind_button_combination_action_internal(action, binding, None)
    }

    pub fn bind_axis<A: Into<TAxisAction>, B: Into<AxisBinding>>(&mut self, action: A, axis_binding: B) -> &mut Self {
        self.bind_axis_with_deadzone(action, axis_binding, 0.)
    }

    pub fn bind_axis_with_deadzone<A: Into<TAxisAction>, B: Into<AxisBinding>>(&mut self, action: A, axis_binding: B, deadzone: f32) -> &mut Self {
        self.bind_axis_with_deadzone_internal(action, axis_binding, deadzone, None)
    }
}

#[cfg(feature = "multiplayer")]
impl<TKeyAction: ActionMapInput, TAxisAction: ActionMapInput> ActionMap<TKeyAction, TAxisAction>
{
    pub fn bind_button_action<K: Into<TKeyAction>, B: Into<ButtonCode>>(&mut self, player_id: usize, action: K, button: B) -> &mut Self {
        self.bind_button_action_internal(action, button, Some(player_id))
    }

    pub fn bind_button_combination_action<K: Into<TKeyAction>, B: IntoIterator<Item = ButtonCode>>(&mut self, player_id: usize, action: K, binding: B) -> &mut Self {
        self.bind_button_combination_action_internal( action, binding, Some(player_id))
    }

    pub fn bind_axis<A: Into<TAxisAction>, B: Into<AxisBinding>>(&mut self, player_id: usize,action: A, axis_binding: B) -> &mut Self {
        self.bind_axis_with_deadzone(player_id, action, axis_binding, 0.)
    }

    pub fn bind_axis_with_deadzone<A: Into<TAxisAction>, B: Into<AxisBinding>>(&mut self, player_id: usize,action: A, axis_binding: B, deadzone: f32) -> &mut Self {
        self.bind_axis_with_deadzone_internal(action, axis_binding, deadzone, Some(player_id))
    }
}

impl<TKeyAction: ActionMapInput, TAxisAction: ActionMapInput> ActionMap<TKeyAction, TAxisAction>
{
    // todo: actually a load from path/assetpath fn that handles the loading?
    pub fn set_bindings(&mut self, map: Self) {
        for action in map.key_action_bindings {
            for b in action.1 {
                self.bind_button_combination_action_internal(action.0.value, b, action.0.id);
            }
        }

        for action in map.axis_action_bindings {
            for b in action.1 {
                self.bind_axis_with_deadzone_internal(action.0.value, b.0, b.1 as f32 / DEADZONE_PRECISION, action.0.id);
            }
        }
    }

    fn bind_button_action_internal<K: Into<TKeyAction>, B: Into<ButtonCode>>(&mut self, action: K, button: B, player_id: Option<usize>) -> &mut Self {
        self.bind_button_combination_action_internal(action, vec![button.into()], player_id)
    }

    // todo: bind should validate actions don't overlap & return result
    fn bind_button_combination_action_internal<K: Into<TKeyAction>, B: IntoIterator<Item = ButtonCode>>(&mut self, action: K, binding: B, player_id: Option<usize>) -> &mut Self {
        let key = PlayerData { value: action.into(), id: player_id };
        let binding: KeyActionBinding = binding.into_iter().collect();

        #[cfg(feature = "validation")]
        {
            if crate::validation::validate(self, player_id, binding.clone()) {
            }
            else {
                // todo: don't panic, just return a result
                panic!("Oh no, a terrible thing has happened: {:?}!", binding);
            }
        }

        if !self.key_action_bindings.contains_key(&key) {
            self.key_action_bindings.insert(key, Default::default());
        }

        if let Some(action) = self.key_action_bindings.get_mut(&key) {
            self.bound_keys.extend(binding.clone().iter().map(|btn| btn.player_data(player_id)));
            action.push(binding);
        }

        self
    }

    // todo: bind should validate actions don't overlap & return result?
    // does that actually apply to axes?
    fn bind_axis_with_deadzone_internal<A: Into<TAxisAction>, B: Into<AxisBinding>>(&mut self, action: A, axis_binding: B, deadzone: f32, player_id: Option<usize>) -> &mut Self {
        let key = PlayerData { value: action.into(), id: player_id };
        if !self.axis_action_bindings.contains_key(&key) {
            self.axis_action_bindings.insert(key, Default::default());
        }

        if let Some(action) = self.axis_action_bindings.get_mut(&key) {
            let mut axis_binding: AxisBinding = axis_binding.into();
            match axis_binding {
                AxisBinding::Buttons(neg_key, pos_key) => {
                    self.bound_keys.insert(neg_key.player_data(player_id));
                    self.bound_keys.insert(pos_key.player_data(player_id));
                },
                AxisBinding::GamepadAxis(axis) => {
                    let mut rebind_to_buttons = |neg: GamepadButtonType, pos: GamepadButtonType| {
                        self.bound_keys.insert(ButtonCode::Gamepad(neg).player_data(player_id));
                        self.bound_keys.insert(ButtonCode::Gamepad(pos).player_data(player_id));
                        axis_binding = AxisBinding::Buttons(neg.into(), pos.into());
                    };

                    if axis == GamepadAxisType::DPadX {
                        rebind_to_buttons(GamepadButtonType::DPadLeft, GamepadButtonType::DPadRight);
                    }
                    else if axis == GamepadAxisType::DPadY {
                        rebind_to_buttons(GamepadButtonType::DPadDown, GamepadButtonType::DPadUp);
                    }
                    else {
                        self.bound_axes.insert(axis);
                    }
                },
            }

            action.insert((axis_binding, (deadzone * DEADZONE_PRECISION) as u32));
        }

        self
    }
}

#[cfg(feature = "multiplayer")]
#[derive(Default)]
pub struct GamepadMap {
    connected_gamepads: HashSet<Gamepad>,
    mapped_gamepads: HashMap<usize, usize>,
}

#[cfg(feature = "multiplayer")]
impl GamepadMap {
    pub fn map_gamepad(&mut self, gamepad_id: usize, player_id: usize) {
        self.mapped_gamepads.insert(gamepad_id, player_id);
    }
}

pub struct ActionInput<TKeyAction, TAxisAction> {
    pub(crate) button_states: HashMap<DeviceData<ButtonCode>, Option<ButtonState>>,
    button_actions: HashMap<PlayerData<TKeyAction>, ActionState>,
    gamepad_axes_values: HashMap<PlayerData<GamepadAxisType>, f32>,
    axes: HashMap<PlayerData<TAxisAction>, f32>,
}

impl<TKeyAction, TAxisAction> Default for ActionInput<TKeyAction, TAxisAction> {
    fn default() -> Self {
        Self { 
            button_states: Default::default(),
            button_actions: Default::default(),
            gamepad_axes_values: Default::default(),
            axes: Default::default(),
        }
    }
}

#[cfg(not(feature = "multiplayer"))]
impl<TKeyAction: ActionMapInput, TAxisAction: ActionMapInput> ActionInput<TKeyAction, TAxisAction>
{
    pub fn get_button_action_state(&self, button: TKeyAction) -> Option<&ActionState> {
        self.get_action_state(&button.into())
    }

    pub fn just_pressed(&self, button: TKeyAction) -> bool {
        self.is_button_action_in_state(button.into(), ActionState::Pressed)
    }

    pub fn held(&self, button: TKeyAction) -> bool {
        self.is_button_action_in_state(button.into(), ActionState::Held(Default::default()))
    }

    pub fn just_released(&self, button: TKeyAction) -> bool {
        self.is_button_action_in_state(button.into(), ActionState::Released(Default::default()))
    }

    pub fn used(&self, button: TKeyAction) -> bool {
        self.is_button_action_in_state(button.into(), ActionState::Used)
    }

    pub fn use_button_action(&mut self, button: TKeyAction) {
        self.button_actions.insert(button.into(), ActionState::Used);
    }

    pub fn get_axis(&self, axis: &TAxisAction) -> f32 {
        if let Some(axis_value) = self.axes.get(&PlayerData::new(*axis)) {
            *axis_value
        }
        else {
            0.
        }
    }

    pub fn get_xy_axes(&self, x_axis: &TAxisAction, y_axis: &TAxisAction) -> Vec2 {
        Vec2::new(self.get_axis(x_axis), self.get_axis(y_axis)).normalize_or_zero()
    }
}

#[cfg(feature = "multiplayer")]
impl<TKeyAction: ActionMapInput, TAxisAction: ActionMapInput> ActionInput<TKeyAction, TAxisAction>
{
    pub fn get_button_action_state(&self, player_id: usize, button: &TKeyAction) -> Option<&ActionState> {
        self.get_action_state(&PlayerData::new_with_id(*button, player_id))
    }

    pub fn just_pressed(&self, player_id: usize, button: TKeyAction) -> bool {
        self.is_button_action_in_state(PlayerData::new_with_id(button, player_id), ActionState::Pressed)
    }

    pub fn held(&self, player_id: usize, button: TKeyAction) -> bool {
        self.is_button_action_in_state(PlayerData::new_with_id(button, player_id), ActionState::Held(Default::default()))
    }

    pub fn just_released(&self, player_id: usize, button: TKeyAction) -> bool {
        self.is_button_action_in_state(PlayerData::new_with_id(button, player_id), ActionState::Released(Default::default()))
    }

    pub fn used(&self, player_id: usize, button: TKeyAction) -> bool {
        self.is_button_action_in_state(PlayerData::new_with_id(button, player_id), ActionState::Used)
    }

    pub fn use_button_action(&mut self, player_id: usize, button: TKeyAction) {
        self.button_actions.insert(PlayerData::new_with_id(button, player_id), ActionState::Used);
    }

    pub fn get_axis(&self, player_id: usize, axis: &TAxisAction) -> f32 {
        if let Some(axis_value) = self.axes.get(&PlayerData::new_with_id(*axis, player_id)) {
            *axis_value
        }
        else {
            0.
        }
    }

    pub fn get_xy_axes(&self, player_id: usize, x_axis: &TAxisAction, y_axis: &TAxisAction) -> Vec2 {
        Vec2::new(self.get_axis(player_id, x_axis), self.get_axis(player_id, y_axis)).normalize_or_zero()
    }
}

impl<TKeyAction: ActionMapInput, TAxisAction: ActionMapInput> ActionInput<TKeyAction, TAxisAction>
{
    fn get_action_state(&self, player_button_action_data: &PlayerData<TKeyAction>) -> Option<&ActionState> {
        self.button_actions.get(player_button_action_data)
    }

    fn is_button_action_in_state(&self, key: PlayerData<TKeyAction>, state: ActionState) -> bool {
        if let Some(key_state) = self.button_actions.get(&key) {
            // compare enum variants (not their data)
            std::mem::discriminant(key_state) == std::mem::discriminant(&state)
        }
        else {
            false
        }
    }

    fn button_is_pressed_or_held(&self, button_data: &PlayerData<ButtonCode>) -> bool {
        if let Some(Some(ButtonState::Pressed | ButtonState::Held)) = self.button_states.get(button_data) {
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
    for btn_data in map.bound_keys.iter() {
        if let PlayerData { value: ButtonCode::Kb(key), .. } = btn_data {
            input.button_states.insert(*btn_data, get_button_state(&kb_input, key));
        }
    }
}

pub(crate) fn handle_mouse_button_events<TKeyAction: ActionMapInput + 'static, TAxisAction: ActionMapInput + 'static>(
    mut input: ResMut<ActionInput<TKeyAction, TAxisAction>>,
    mouse_input: Res<Input<MouseButton>>,
    map: Res<ActionMap<TKeyAction, TAxisAction>>
) {
    for btn_data in map.bound_keys.iter() {
        if let PlayerData { value: ButtonCode::Mouse(button), .. } = btn_data {
            input.button_states.insert(*btn_data, get_button_state(&mouse_input, button));
        }
    }
}

pub(crate) fn handle_gamepad_events<TKeyAction: ActionMapInput + 'static, TAxisAction: ActionMapInput + 'static>(
    mut gamepad_events: EventReader<GamepadEvent>,
    mut input: ResMut<ActionInput<TKeyAction, TAxisAction>>,
    #[cfg(feature = "multiplayer")]
    mut gamepad_map: ResMut<GamepadMap>,
    map: Res<ActionMap<TKeyAction, TAxisAction>>,
    gamepad_input: Res<Input<GamepadButton>>,)
{
    for event in gamepad_events.iter() {
        match event {
            GamepadEvent(gamepad, GamepadEventType::ButtonChanged(button, _strength)) => {
                let input_code = ButtonCode::Gamepad(*button);
                    #[cfg(feature = "multiplayer")]
                    {
                        if let Some(player_id) = gamepad_map.mapped_gamepads.get(&gamepad.0) {
                            if map.bound_keys.get(&input_code.player_data(Some(*player_id))).is_some() {
                                input.button_states.insert(input_code.player_data(Some(*player_id)), get_button_state(&gamepad_input, &GamepadButton(*gamepad, *button)));
                            }
                        }
                    }
                    #[cfg(not(feature = "multiplayer"))]
                    {
                        if map.bound_keys.get(&input_code.player_data(None)).is_some() {
                            input.button_states.insert(input_code.player_data(None), get_button_state(&gamepad_input, &GamepadButton(*gamepad, *button)));
                        }
                    }
            }
            GamepadEvent(gamepad, GamepadEventType::AxisChanged(axis_type, strength)) => {
                if map.bound_axes.get(axis_type).is_some() {
                    #[cfg(feature = "multiplayer")]
                    {
                        if let Some(player_id) = gamepad_map.mapped_gamepads.get(&gamepad.0) {
                            input.gamepad_axes_values.insert(DeviceData::new_with_id(*axis_type, *player_id), *strength);
                        }
                    }
                    #[cfg(not(feature = "multiplayer"))]
                    {
                        input.gamepad_axes_values.insert(DeviceData { value: *axis_type, id: None }, *strength);
                    }
                }
            }
            #[cfg(feature = "multiplayer")]
            GamepadEvent(gamepad, GamepadEventType::Connected) => {
                gamepad_map.connected_gamepads.insert(*gamepad);
                println!("Gamepad Connected: {}", gamepad.0);
            }
            #[cfg(feature = "multiplayer")]
            GamepadEvent(gamepad, GamepadEventType::Disconnected) => {
                gamepad_map.connected_gamepads.remove(gamepad);
            }
            _ => {}
        }
    }
}

pub(crate) fn process_button_actions<TKeyAction: ActionMapInput + 'static, TAxisAction: ActionMapInput + 'static>(
    mut input: ResMut<ActionInput<TKeyAction, TAxisAction>>,
    map: Res<ActionMap<TKeyAction, TAxisAction>>,
    time: Res<Time>,
) {
    'actions: for (action_data, bindings) in map.key_action_bindings.iter() {
        let current_state = input.get_action_state(&action_data);
        let current_duration = current_state.unwrap_or(&ActionState::Used).duration();
        match current_state {
            None | Some(ActionState::Released(..) | ActionState::Used) => {
                'bindings: for binding_keys in bindings {
                    let mut just_pressed_at_least_one_key = false;

                    for k in binding_keys.iter() {
                        if let Some(key_state) = input.button_states.get(&k.player_data(action_data.id)) {
                            match key_state {
                                Some(ButtonState::Pressed) => {
                                    just_pressed_at_least_one_key = true;
                                    continue;
                                },
                                Some(ButtonState::Held) => {
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
                        input.button_actions.insert(*action_data, ActionState::Pressed);
                        continue 'actions;
                    }
                }

                input.button_actions.remove(&action_data);
            },
            Some(ActionState::Pressed | ActionState::Held(..)) => {
                // check if all keys are still held
                'held_bindings: for binding_keys in bindings {
                    for k in binding_keys.iter() {
                        if let Some(key_state) = input.button_states.get(&k.player_data(action_data.id)) {
                            match key_state {
                                Some(ButtonState::Pressed | ButtonState::Held) => {
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

                    input.button_actions.insert(*action_data, ActionState::Held(ActiveKeyData {
                        duration: current_duration + time.delta_seconds(),
                    }));
                    continue 'actions;
                }

                input.button_actions.insert(*action_data, ActionState::Released(ActiveKeyData {
                    duration: current_duration + time.delta_seconds(),
                }));
            },
        }
    }
}

pub(crate) fn process_axis_actions<TKeyAction: ActionMapInput + 'static, TAxisAction: ActionMapInput + 'static>(
    mut input: ResMut<ActionInput<TKeyAction, TAxisAction>>,
    map: Res<ActionMap<TKeyAction, TAxisAction>>
) {
    for (axis_action_data, bindings) in &map.axis_action_bindings {
        let axis_value = bindings.iter().map(|b| {
            let (val, deadzone) = match b {
                (AxisBinding::Buttons(neg, pos), _) => {
                    let mut val = 0.;
                    if input.button_is_pressed_or_held(&neg.player_data(axis_action_data.id)) {
                        val -= 1.;
                    }
                    if input.button_is_pressed_or_held(&pos.player_data(axis_action_data.id)) {
                        val += 1.;
                    }

                    (val, 0)
                },
                (AxisBinding::GamepadAxis(gamepad_axis), deadzone) => {
                    let axis_data = PlayerData {
                        value: *gamepad_axis,
                        id: axis_action_data.id,
                    };

                    (*input.gamepad_axes_values.get(&axis_data).unwrap_or(&0.), *deadzone)
                },
            };

            if deadzone == 0 {
                val
            }
            else {
                let deadzone = deadzone as f32 / DEADZONE_PRECISION;
                if val.abs() > deadzone {
                    // normalize the value back to the 0.0..1.0 range
                    let normalized_value = (val.abs() - 1.) / deadzone + 1.;
                    normalized_value * val.signum()
                }
                else {
                    0.
                }
            }
        }).fold(0., |a: f32, b: f32| if a.abs() > b.abs() { a } else { b });

        input.axes.insert(*axis_action_data, axis_value);
    }
}

fn get_button_state<T: Copy + Eq + Hash>(
    input: &Input<T>,
    button: &T
) -> Option<ButtonState> {
    if input.just_pressed(*button) {
        Some(ButtonState::Pressed)
    }
    else if input.just_released(*button) {
        Some(ButtonState::Released)
    }
    else if input.pressed(*button) {
        Some(ButtonState::Held)
    }
    else {
        None
    }
}
