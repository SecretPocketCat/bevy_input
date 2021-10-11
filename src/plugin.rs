use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};
use maplit::{hashset};
use bevy::{
    input::{
        gamepad::{GamepadAxisType, GamepadEvent, GamepadEventType},
        InputSystem,
    },
    prelude::*,
};

pub struct ActionPlugin<'a, T>(std::marker::PhantomData<&'a T>);

impl<'a, T> Default for ActionPlugin<'a, T> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<'a, T> Plugin for ActionPlugin<'a, T>
where
    InputMap<T>: Default,
    T: Hash + Eq + Clone + Send + Sync + Debug,
    'a: 'static,
{
    fn build(&self, app: &mut App) {
        const UPDATE_STATES_LABEL: &str = "UPDATE_STATES";
        app.init_resource::<InputMap<T>>()
            .add_system_to_stage(
                CoreStage::PreUpdate,
                InputMap::<T>::handle_key_events
                    .label(UPDATE_STATES_LABEL)
                    .after(InputSystem),
            )
            .add_system_to_stage(
                CoreStage::PreUpdate,
                InputMap::<T>::handle_gamepad_events
                    .label(UPDATE_STATES_LABEL)
                    .after(InputSystem),
            )
            .add_system_to_stage(
                CoreStage::PreUpdate,
                InputMap::<T>::gamepad_button_input
                    .after(UPDATE_STATES_LABEL),
            )
            .add_system_to_stage(
                CoreStage::PreUpdate,
                InputMap::<T>::gamepad_axis_input
                    .after(UPDATE_STATES_LABEL),
            );
    }
}
