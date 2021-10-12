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

use crate::action_map::{ActionInput, ActionMap, ActionMapInput, handle_keyboard_button_events, handle_mouse_button_events, process_key_actions};

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
        const UPDATE_STATES_LABEL: &str = "UPDATE_STATES";
        app
        .init_resource::<ActionMap<TKeyAction, TAxisAction>>()
        .init_resource::<ActionInput<TKeyAction, TAxisAction>>()
            .add_system_to_stage(
                CoreStage::PreUpdate,
                handle_keyboard_button_events::<TKeyAction, TAxisAction>
                    .label(UPDATE_STATES_LABEL)
                    .after(InputSystem),
            )
            .add_system_to_stage(
                CoreStage::PreUpdate,
                handle_mouse_button_events::<TKeyAction, TAxisAction>
                    .label(UPDATE_STATES_LABEL)
                    .after(InputSystem),
            )
            .add_system_to_stage(
                CoreStage::PreUpdate,
                process_key_actions::<TKeyAction, TAxisAction>
                    .after(UPDATE_STATES_LABEL),
            );
    }
}
