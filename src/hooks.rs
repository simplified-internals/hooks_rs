use crate::fiber::CURRENT_FIBER;
use std::any::Any;

pub(crate) enum Hooks {
    None,
    UseState { value: Box<dyn Any> },
}

pub fn use_state<S: 'static + Clone>(initial: impl FnOnce() -> S) -> (S, impl Fn(S)) {
    CURRENT_FIBER.with(|f| {
        let fiber_state = unsafe { &mut *f.borrow().expect("Hook was called outside of a fiber.") };

        let idx = fiber_state.hook_index;
        fiber_state.hook_index += 1;

        if idx >= fiber_state.hooks.len() {
            // new hook: create it with this caller location as ID
            fiber_state.hooks.push(Hooks::UseState {
                value: Box::new(initial()),
            });
        }

        let state = match &fiber_state.hooks[idx] {
            Hooks::UseState { value, .. } => value.downcast_ref::<S>().unwrap().clone(),
            _ => panic!("Expected same hooks as previous call. This may be happening if you are calling hooks conditionally")
        };

        let setter = move |new_state: S| {
            CURRENT_FIBER.with(|f| {
                let fiber_state = unsafe { &mut *f.borrow().unwrap() };

                let Hooks::UseState { value } = &mut fiber_state.hooks[idx] else {
                    unreachable!("Hook at this index is not UseState");
                };

                *value = Box::new(new_state);
            });
        };

        (state, setter)
    })
}
