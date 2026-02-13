// Hooks implementations
mod use_effect;
mod use_ref;
mod use_state;

pub use use_effect::*;
pub use use_ref::*;
pub use use_state::*;

/// Internal Hooks enum
use std::any::{Any, TypeId};

pub struct Hook {
    pub(crate) type_id: TypeId,
    pub(crate) state: Box<dyn Any>,
}

use crate::fiber::{CURRENT_FIBER_STATE, FiberState};
pub(crate) fn read_fiber_state(msg: &str) -> &'static mut FiberState {
    CURRENT_FIBER_STATE.with(|f| {
        let fiber_state = unsafe { &mut *f.borrow().expect(msg) };
        fiber_state
    })
}
