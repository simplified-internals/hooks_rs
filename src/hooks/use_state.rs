use crate::{
    fiber::{CURRENT_FIBER_STATE, FiberState},
    hooks::Hooks,
};

#[track_caller]
pub fn use_state<S>(initial: impl FnOnce() -> S) -> (S, SetStateAction<S>)
where
    S: 'static + Clone,
{
    let location = std::panic::Location::caller();

    let fiber_state = CURRENT_FIBER_STATE.with(|f| unsafe {
        &mut *f.borrow().expect(&format!(
            "Hook `use_state` was called outside of a fiber. ({})",
            location
        ))
    });

    let idx = fiber_state.hook_index;
    fiber_state.hook_index += 1;

    // Initialize new hook if needed
    if idx >= fiber_state.hooks.len() {
        fiber_state.hooks.push(Hooks::UseState {
            value: Box::new(initial()),
        });
    }

    let state = {
        match &fiber_state.hooks[idx] {
            Hooks::UseState { value } => value.downcast_ref::<S>().unwrap().clone(),
            other => panic!(
                "Expected `use_state` hook, but got `{other}`. This may happen when calling hooks conditionally. ({})",
                location
            ),
        }
    };

    // Create a setter that is fully independent of CURRENT_FIBER
    let setter = SetStateAction::<S> {
        fiber_ptr: &mut *fiber_state,
        hook_index: idx,
        _marker: std::marker::PhantomData,
    };

    (state, setter)
}

#[derive(Clone)]
pub struct SetStateAction<S> {
    fiber_ptr: *mut FiberState,
    hook_index: usize,
    _marker: std::marker::PhantomData<S>,
}

impl<S: Clone + 'static> SetStateAction<S> {
    fn set(&self, f: &dyn Fn(&S) -> S) {
        unsafe {
            let fiber = &mut *self.fiber_ptr;
            let Hooks::UseState { value } = &mut fiber.hooks[self.hook_index] else {
                unreachable!()
            };
            let current = value.downcast_ref::<S>().unwrap();
            *value = Box::new(f(current));
        }
    }
}

impl<S: Clone + 'static> FnOnce<(&dyn Fn(&S) -> S,)> for SetStateAction<S> {
    type Output = ();

    extern "rust-call" fn call_once(self, args: (&dyn Fn(&S) -> S,)) -> Self::Output {
        self.set(args.0)
    }
}

impl<S: Clone + 'static> FnMut<(&dyn Fn(&S) -> S,)> for SetStateAction<S> {
    extern "rust-call" fn call_mut(&mut self, args: (&dyn Fn(&S) -> S,)) -> Self::Output {
        self.set(args.0)
    }
}

impl<S: Clone + 'static> Fn<(&dyn Fn(&S) -> S,)> for SetStateAction<S> {
    extern "rust-call" fn call(&self, args: (&dyn Fn(&S) -> S,)) -> Self::Output {
        self.set(args.0)
    }
}
