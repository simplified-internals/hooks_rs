use std::{cell::RefCell, rc::Rc};

use crate::hooks::{Hooks, read_fiber_state};

pub fn use_ref<S: 'static>(initial_value: S) -> Rc<RefCell<S>> {
    let location = std::panic::Location::caller();

    let fiber_state = read_fiber_state(&format!(
        "Hook `use_ref` was called outside of a fiber. ({})",
        location
    ));

    let idx = fiber_state.hook_index;
    fiber_state.hook_index += 1;

    if idx >= fiber_state.hooks.len() {
        // MOUNT LOGIC HERE
        let rc = Rc::new(RefCell::new(initial_value));
        fiber_state.hooks.push(Hooks::UseRef {
            current: rc.clone(),
        });
        return rc;
    }

    // UPDATE LOGIC HERE
    match &fiber_state.hooks[idx] {
        Hooks::UseRef { current } => current.clone().downcast::<RefCell<S>>().unwrap(),
        other => panic!("Expected `use_ref` hook, but got `{other}`. ({})", location),
    }
}
