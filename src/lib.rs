#![feature(destructuring_assignment)]
#![feature(if_let_guard)]
#![feature(trait_alias)]

#![warn(clippy::pedantic)]
#![allow(clippy::default_trait_access)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::cast_precision_loss)]


mod action_map;
mod macros;
mod plugin;
#[cfg(feature = "validation")]
mod validation;

pub use action_map::{ActionInput, ActionMap, ActionMapInput, AxisBinding, ButtonCode};
pub use macros::*;
pub use plugin::ActionInputPlugin;
#[cfg(feature = "validation")]
pub use validation::BindingError;
#[cfg(feature = "multiplayer")]
pub use action_map::GamepadMap;
