use std::{cell::RefCell, rc::Rc};

use crate::{fiber::CURRENT_FIBER_STATE, hooks::Hooks};

pub fn use_ref<S: 'static>(initial_value: S) -> Rc<RefCell<S>> {
    let location = std::panic::Location::caller();

    CURRENT_FIBER_STATE.with(|f| {
        let fiber_state = unsafe {
            &mut *f.borrow().expect(&format!(
                "Hook `use_effect` was called outside of a fiber. ({})",
                location
            ))
        };

        let idx = fiber_state.hook_index;
        fiber_state.hook_index += 1;

        // New hook
        if idx >= fiber_state.hooks.len() {
            let rc = Rc::new(RefCell::new(initial_value));
            fiber_state.hooks.push(Hooks::UseRef {
                current: rc.clone(),
            });
            return rc;
        }

        match &fiber_state.hooks[idx] {
            Hooks::UseRef { current } => current.clone().downcast::<RefCell<S>>().unwrap(),
            other => panic!("Expected `use_ref` hook, but got `{other}`. ({})", location),
        }
    })
}
