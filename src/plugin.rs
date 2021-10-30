use bevy::{input::InputSystem, prelude::*};
#[cfg(feature = "serialize")]
use bevy_extensions::panic_on_error;

use crate::{MapIoEvent, action_map::{
    handle_gamepad_events, handle_keyboard_button_events, handle_mouse_button_events,
    process_axis_actions, process_button_actions, ActionInput, ActionMap, ActionMapInput,
}, bindings_loader::{ActionMapLoad, ActionMapSave, MapIoRequest, load_map, process_map_event, save_map}};

pub struct ActionInputPlugin<'a, TKeyAction, TAxisAction> {
    _key_t: std::marker::PhantomData<&'a TKeyAction>,
    _axis_t: std::marker::PhantomData<&'a TAxisAction>,
}

impl<'a, TKeyAction, TAxisAction> Default for ActionInputPlugin<'a, TKeyAction, TAxisAction> {
    fn default() -> Self {
        Self { 
            _key_t: Default::default(), 
            _axis_t: Default::default(),
        }
    }
}

impl<'a, TKeyAction, TAxisAction> Plugin for ActionInputPlugin<'a, TKeyAction, TAxisAction>
where
    ActionMap<TKeyAction, TAxisAction>: Default,
    TKeyAction: ActionMapInput + serde::Serialize + for<'de> serde::Deserialize<'de>,
    TAxisAction: ActionMapInput + serde::Serialize + for<'de> serde::Deserialize<'de>,
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
                .insert_resource(ActionMapLoad::<TKeyAction, TAxisAction>(None))
                .insert_resource(ActionMapSave(None))
                .add_event::<MapIoRequest>()
                .add_event::<MapIoEvent>()
                .add_system(process_map_event::<TKeyAction, TAxisAction>)
                .add_system(load_map::<TKeyAction, TAxisAction>
                    .chain(panic_on_error))
                .add_system(save_map
                    .chain(panic_on_error));
        }
    }
}
