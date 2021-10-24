use bevy::{input::InputSystem, prelude::*};

use crate::action_map::{
    handle_gamepad_events, handle_keyboard_button_events, handle_mouse_button_events,
    process_axis_actions, process_button_actions, ActionInput, ActionMap, ActionMapInput,
};

pub struct ActionInputPlugin<'a, TKeyAction, TAxisAction>(
    std::marker::PhantomData<&'a TKeyAction>,
    std::marker::PhantomData<&'a TAxisAction>,
);

impl<'a, TKeyAction, TAxisAction> ActionInputPlugin<'a, TKeyAction, TAxisAction> {
    pub fn new() -> Self {
        Self(std::marker::PhantomData, std::marker::PhantomData)
    }
}

impl<'a, TKeyAction, TAxisAction> Default for ActionInputPlugin<'a, TKeyAction, TAxisAction> {
    fn default() -> Self {
        Self::new()
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
        app.init_resource::<ActionMap<TKeyAction, TAxisAction>>()
            .init_resource::<ActionInput<TKeyAction, TAxisAction>>()
            .add_system_to_stage(
                CoreStage::PreUpdate,
                handle_keyboard_button_events::<TKeyAction, TAxisAction>
                    .label(PROCESS_INPUT_LABEL)
                    .after(InputSystem),
            )
            .add_system_to_stage(
                CoreStage::PreUpdate,
                handle_mouse_button_events::<TKeyAction, TAxisAction>
                    .label(PROCESS_INPUT_LABEL)
                    .after(InputSystem),
            )
            .add_system_to_stage(
                CoreStage::PreUpdate,
                handle_gamepad_events::<TKeyAction, TAxisAction>
                    .label(PROCESS_INPUT_LABEL)
                    .after(InputSystem),
            )
            .add_system_to_stage(
                CoreStage::PreUpdate,
                process_button_actions::<TKeyAction, TAxisAction>.after(PROCESS_INPUT_LABEL),
            )
            .add_system_to_stage(
                CoreStage::PreUpdate,
                process_axis_actions::<TKeyAction, TAxisAction>.after(PROCESS_INPUT_LABEL),
            );

        #[cfg(feature = "multiplayer")]
        {
            app.init_resource::<crate::action_map::GamepadMap>();
        }
    }
}
