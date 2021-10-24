#![feature(destructuring_assignment)]
#![feature(if_let_guard)]
#![feature(trait_alias)]
#![allow(clippy::type_complexity)]

mod action_map;
mod macros;
mod plugin;

#[cfg(feature = "validation")]
mod validation;

pub use action_map::{ActionInput, ActionMap, ActionMapInput, AxisBinding, ButtonCode};
pub use macros::*;
pub use plugin::ActionInputPlugin;
pub use validation::BindingError;

#[cfg(feature = "multiplayer")]
pub use action_map::GamepadMap;
