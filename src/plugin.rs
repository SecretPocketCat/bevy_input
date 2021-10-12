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

use crate::action_map::{ActionInput, ActionMap, ActionMapInput, handle_keyboard_button_events, handle_mouse_button_events, handle_gamepad_events, process_button_actions, process_axis_actions};

pub struct ActionInputPlugin<'a, TKeyAction, TAxisAction>(std::marker::PhantomData<&'a TKeyAction>, std::marker::PhantomData<&'a TAxisAction>);

impl<'a, TKeyAction, TAxisAction> ActionInputPlugin<'a, TKeyAction, TAxisAction> {
    pub fn new() -> Self {
        Self(std::marker::PhantomData, std::marker::PhantomData)
    }
}

impl<'a, TKeyAction, TAxisAction> Plugin for ActionInputPlugin<'a, TKeyAction, TAxisAction>
where
    ActionMap<TKeyAction, TAxisAction>: Default,
    TKeyAction: ActionMapInput,
    TAxisAction: ActionMapInput,
    'a: 'static,
{
    fn build(&self, app: &mut App) {
        const PROCESS_INPUT_LABEL: &str = "UPDATE_STATES";
        app
        .init_resource::<ActionMap<TKeyAction, TAxisAction>>()
        .init_resource::<ActionInput<TKeyAction, TAxisAction>>()
            .add_system_to_stage(
                CoreStage::PreUpdate,
                handle_keyboard_button_events::<TKeyAction, TAxisAction>
                    .label(PROCESS_INPUT_LABEL)
                    .after(InputSystem),
            ).add_system_to_stage(
                CoreStage::PreUpdate,
                handle_mouse_button_events::<TKeyAction, TAxisAction>
                    .label(PROCESS_INPUT_LABEL)
                    .after(InputSystem),
            ).add_system_to_stage(
                CoreStage::PreUpdate,
                handle_gamepad_events::<TKeyAction, TAxisAction>
                    .label(PROCESS_INPUT_LABEL)
                    .after(InputSystem),
            ).add_system_to_stage(
                CoreStage::PreUpdate,
                process_button_actions::<TKeyAction, TAxisAction>
                    .after(PROCESS_INPUT_LABEL),
            ).add_system_to_stage(
                CoreStage::PreUpdate,
                process_axis_actions::<TKeyAction, TAxisAction>
                    .after(PROCESS_INPUT_LABEL),
            );
    }
}
