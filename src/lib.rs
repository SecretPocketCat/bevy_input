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
mod app_ext;
#[cfg(feature = "serialize")]
mod bindings_loader;
mod macros;
mod validation;

pub use action_map::{
    ActionInput, ActionMap, ActionMapInput, ActionState, AxisBinding, ButtonCode, InputGamepad,
};
pub use app_ext::AppActionInputExt;
#[cfg(feature = "serialize")]
pub use bindings_loader::{MapIoEvent, MapIoRequest};
pub use macros::*;
pub use validation::BindingError;
