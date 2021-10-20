#![feature(destructuring_assignment)]
#![feature(if_let_guard)]
#![feature(trait_alias)]

mod plugin;
mod action_map;
mod macros;

#[cfg(feature = "validation")]
mod validation;

pub use action_map::{ActionInput, ActionMap, ActionMapInput, AxisBinding, ButtonCode};
pub use plugin::ActionInputPlugin;
pub use macros::*;

#[cfg(feature = "multiplayer")]
pub use action_map::{GamepadMap};
