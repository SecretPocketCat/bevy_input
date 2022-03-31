use bevy::{input::InputSystem, prelude::*};
#[cfg(feature = "serialize")]
use bevy_extensions::panic_on_error;

use crate::action_map::add_input;
#[cfg(feature = "serialize")]
use crate::{
    action_map::{
        handle_gamepad_events, handle_keyboard_input, handle_mouse_input, process_axis_actions,
        process_button_actions, ActionInput, ActionMap, ActionMapInput,
    },
    bindings_loader::{
        load_map, process_map_event, save_map, ActionMapLoad, ActionMapSave, MapIoRequest,
    },
    MapIoEvent,
};

#[cfg(not(feature = "serialize"))]
use crate::action_map::{
    handle_gamepad_events, handle_keyboard_input, handle_mouse_input, process_axis_actions,
    process_button_actions, ActionInput, ActionMap, ActionMapInput,
};

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
    TKeyAction: ActionMapInput,
    TAxisAction: ActionMapInput,
    'a: 'static,
{
    fn build(&self, app: &mut App) {
        const PROCESS_INPUT_LABEL: &str = "UPDATE_STATES";
        app.add_system_to_stage(CoreStage::Last, add_input::<TKeyAction, TAxisAction>)
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::new()
                    .with_system(
                        handle_keyboard_input::<TKeyAction, TAxisAction>
                            .label(PROCESS_INPUT_LABEL)
                            .after(InputSystem),
                    )
                    .with_system(
                        handle_mouse_input::<TKeyAction, TAxisAction>
                            .label(PROCESS_INPUT_LABEL)
                            .after(InputSystem),
                    )
                    .with_system(
                        handle_gamepad_events::<TKeyAction, TAxisAction>
                            .label(PROCESS_INPUT_LABEL)
                            .after(InputSystem),
                    )
                    .with_system(
                        process_button_actions::<TKeyAction, TAxisAction>
                            .after(PROCESS_INPUT_LABEL),
                    )
                    .with_system(
                        process_axis_actions::<TKeyAction, TAxisAction>.after(PROCESS_INPUT_LABEL),
                    ),
            );

        #[cfg(feature = "serialize")]
        {
            app.insert_resource(ActionMapLoad::<TKeyAction, TAxisAction>(None))
                .insert_resource(ActionMapSave(None))
                .add_event::<MapIoRequest>()
                .add_event::<MapIoEvent>()
                .add_system(process_map_event::<TKeyAction, TAxisAction>)
                .add_system(load_map::<TKeyAction, TAxisAction>.chain(panic_on_error))
                .add_system(save_map.chain(panic_on_error));
        }
    }
}
