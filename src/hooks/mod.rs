// Hooks implementations
pub mod use_context;
pub mod use_effect;
pub mod use_ref;
pub mod use_state;

/// Internal Hooks enum
use std::any::{Any, TypeId};

pub struct Hook {
    pub type_id: TypeId,
    pub state: Box<dyn Any>,
}

use crate::fiber::{CURRENT_FIBER_ID, FIBER_TREE, HooksState};

/// Returns the current fiber's state by resolving the active fiber id.
pub fn read_fiber_state(msg: &str) -> &'static mut HooksState {
    let id = CURRENT_FIBER_ID
        .with(|cell| cell.borrow().clone())
        .unwrap_or_else(|| panic!("{msg}"));

    let fiber_rc = FIBER_TREE.with(|t| {
        let tree = t.borrow();
        let node = tree.0.get(&id).unwrap_or_else(|| panic!("{msg}"));
        node.fiber.clone()
    });

    let state_ptr = {
        let mut fiber_any = fiber_rc.borrow_mut();
        fiber_any.state_ptr_mut()
    };

    unsafe { &mut *state_ptr }
}
