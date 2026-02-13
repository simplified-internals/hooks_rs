use std::{any::TypeId, cell::RefCell, rc::Rc};

use crate::hooks::{Hook, read_fiber_state};

pub(crate) struct UseRef<S> {
    current: Rc<RefCell<S>>,
}

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
        fiber_state.hooks.push(Hook {
            type_id: TypeId::of::<UseRef<S>>(),
            state: Box::new(UseRef {
                current: rc.clone(),
            }),
        });
        return rc;
    }

    // UPDATE LOGIC HERE
    let hook = &fiber_state.hooks[idx];
    if hook.type_id != TypeId::of::<UseRef<S>>() {
        panic!("Expected `use_ref` hook, but got `{:?}`.", hook.type_id);
    }

    let use_ref = hook.state.downcast_ref::<UseRef<S>>().unwrap();
    use_ref.current.clone()
}
