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
use bevy::{input::InputSystem, prelude::*};
#[cfg(feature = "serialize")]
use bevy_extensions::panic_on_error;

pub const PROCESS_INPUT_LABEL: &str = "UPDATE_STATES";

#[cfg(not(feature = "serialize"))]
use crate::action_map::{
    handle_gamepad_events, handle_keyboard_input, handle_mouse_input, process_axis_actions,
    process_button_actions, ActionInput, ActionMap, ActionMapInput,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct NoAxis;

// todo:  fix serialization
pub trait AppActionInputExt {
    fn add_action_input_systems<TKeyAction>(&mut self) -> &mut Self
    where
        ActionMap<TKeyAction, NoAxis>: Default,
        TKeyAction: ActionMapInput + 'static;

    fn add_action_input_systems_with_axis<TKeyAction, TAxisAction>(&mut self) -> &mut Self
    where
        ActionMap<TKeyAction, TAxisAction>: Default,
        TKeyAction: ActionMapInput + 'static,
        TAxisAction: ActionMapInput + 'static;
}

impl AppActionInputExt for App {
    fn add_action_input_systems_with_axis<TKeyAction, TAxisAction>(&mut self) -> &mut Self
    where
        ActionMap<TKeyAction, TAxisAction>: Default,
        TKeyAction: ActionMapInput + 'static,
        TAxisAction: ActionMapInput + 'static,
    {
        self.add_system_to_stage(CoreStage::Last, add_input::<TKeyAction, TAxisAction>)
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
            self.insert_resource(ActionMapLoad::<TKeyAction, TAxisAction>(None))
                .insert_resource(ActionMapSave(None))
                .add_event::<MapIoRequest>()
                .add_event::<MapIoEvent>()
                .add_system(process_map_event::<TKeyAction, TAxisAction>)
                .add_system(load_map::<TKeyAction, TAxisAction>.chain(panic_on_error))
                .add_system(save_map.chain(panic_on_error));
        }

        self
    }

    fn add_action_input_systems<TKeyAction>(&mut self) -> &mut Self
    where
        ActionMap<TKeyAction>: Default,
        TKeyAction: ActionMapInput + 'static,
    {
        self.add_action_input_systems_with_axis::<TKeyAction, NoAxis>()
    }
}
