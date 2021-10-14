#![feature(destructuring_assignment)]
#![feature(if_let_guard)]

mod plugin;
mod action_map;

pub use action_map::{ActionInput, ActionMap, ActionMapInput};
pub use plugin::ActionInputPlugin;
pub use crate::action_map::AxisBinding;

#[cfg(feature = "multiplayer")]
pub use action_map::{GamepadMap};
