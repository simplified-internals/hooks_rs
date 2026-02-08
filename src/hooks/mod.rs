// Hooks implementations
mod use_effect;
mod use_ref;
mod use_state;

pub use use_effect::*;
pub use use_ref::*;
pub use use_state::*;

use crate::utils::DynEq;

/// Internal Hooks enum
use std::{any::Any, fmt::Display, rc::Rc};

pub(crate) enum Hooks {
    UseEffect { deps: Vec<Box<dyn DynEq>> },
    UseState { value: Box<dyn Any> },
    UseRef { current: Rc<dyn Any> },
}

impl Display for Hooks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Hooks::UseState { value: _ } => f.write_str("use_state(..)"),
            Hooks::UseEffect { deps: _ } => f.write_str("use_effect(..)"),
            Hooks::UseRef { current: _ } => f.write_str("use_ref(..)"),
        }
    }
}
