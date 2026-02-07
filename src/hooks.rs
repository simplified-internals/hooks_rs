use crate::{
    fiber::CURRENT_FIBER,
    utils::{DynEq, deps_changed},
};
use std::{any::Any, cell::RefCell, fmt::Display, rc::Rc};

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

#[track_caller]
pub fn use_state<S>(initial: impl FnOnce() -> S) -> (S, impl Fn(S))
where
    S: 'static + Clone,
{
    let location = std::panic::Location::caller();

    CURRENT_FIBER.with(|f| {
        let fiber_state = unsafe {
            &mut *f.borrow().expect(&format!(
                "Hook `use_state` was called outside of a fiber. ({})",
                location
            ))
        };

        let idx = fiber_state.hook_index;
        fiber_state.hook_index += 1;

        // New hook
        if idx >= fiber_state.hooks.len() {
            fiber_state.hooks.push(Hooks::UseState {
                value: Box::new(initial()),
            });
        }

        let state = match &fiber_state.hooks[idx] {
            Hooks::UseState { value, .. } => value.downcast_ref::<S>().unwrap().clone(),
            other => panic!("Expected `use_state` hook, but got `{other}`. This may happen when calling hooks conditionally. ({})",location),
        };

        let setter = move |new_state: S| {
            CURRENT_FIBER.with(|f| {
                let fiber_state = unsafe { &mut *f.borrow().unwrap() };

                let Hooks::UseState { value } = &mut fiber_state.hooks[idx] else {
                    unreachable!();
                };

                *value = Box::new(new_state);
            });
        };

        (state, setter)
    })
}

#[track_caller]
pub fn use_effect(effect: &mut impl FnMut(), deps: Vec<Box<dyn DynEq>>) {
    let location = std::panic::Location::caller();

    CURRENT_FIBER.with(|f| {
        let fiber_state = unsafe {
            &mut *f.borrow().expect(&format!(
                "Hook `use_effect` was called outside of a fiber. ({})",
                location
            ))
        };

        let idx = fiber_state.hook_index;
        fiber_state.hook_index += 1;

        if idx >= fiber_state.hooks.len() {
            fiber_state.hooks.push(Hooks::UseEffect { deps });
            effect();
            return;
        }

        let prev_deps = match &mut fiber_state.hooks[idx] {
            Hooks::UseEffect { deps: prev_deps } => prev_deps,
            other => panic!("Expected `use_hook` hook, but got `{other}`. This may happen when calling hooks conditionally. ({})", location)
        };

        if deps_changed(prev_deps,&deps) {
            effect();
            *prev_deps = deps;
        }
    })
}

pub fn use_ref<S: 'static>(initial_value: S) -> Rc<RefCell<S>> {
    let location = std::panic::Location::caller();

    CURRENT_FIBER.with(|f| {
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
