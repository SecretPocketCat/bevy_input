use std::marker::PhantomData;

use bevy::{input::InputSystem, prelude::*};

use crate::{action_map::{
    handle_gamepad_events, handle_keyboard_button_events, handle_mouse_button_events,
    process_axis_actions, process_button_actions, ActionInput, ActionMap, ActionMapInput,
}, bindings_loader::process_binding_assets};

pub struct ActionInputPlugin<'a, TKeyAction, TAxisAction> {
    _key_t: std::marker::PhantomData<&'a TKeyAction>,
    _axis_t: std::marker::PhantomData<&'a TAxisAction>,
    #[cfg(feature = "serialize")]
    path: String,
}

impl<'a, TKeyAction, TAxisAction> ActionInputPlugin<'a, TKeyAction, TAxisAction> {
    #[cfg(not(feature = "serialize"))]
    pub fn new() -> Self {
        Self{ 
            _key_t: std::marker::PhantomData,
            _axis_t: std::marker::PhantomData
        }
    }

    #[cfg(feature = "serialize")]
    pub fn new(path: String) -> Self {
        Self{ 
            _key_t: std::marker::PhantomData,
            _axis_t: std::marker::PhantomData,
            path,
        }
    }
}

impl<'a, TKeyAction, TAxisAction> Plugin for ActionInputPlugin<'a, TKeyAction, TAxisAction>
where
    ActionMap<TKeyAction, TAxisAction>: Default,
    TKeyAction: ActionMapInput + for<'de> serde::Deserialize<'de>,
    TAxisAction: ActionMapInput + for<'de> serde::Deserialize<'de>,
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

        #[cfg(feature = "serialize")]
        {
            app
                .add_asset::<crate::action_map::SerializedActionMap<TKeyAction, TAxisAction>>()
                .add_asset_loader(crate::bindings_loader::BindingsLoader::<TKeyAction, TAxisAction>::new())
                .add_system(process_binding_assets::<TKeyAction, TAxisAction>);
        }
    }
}
