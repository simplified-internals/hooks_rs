use std::any::Any;

use crate::fiber::{FiberState, state::CURRENT_FIBER_STATE};

pub struct Fiber<P, R> {
    pub(crate) fun: fn(P) -> R,
    pub(crate) state: FiberState,
    pub(crate) is_dirty: bool,
}

impl<P, R> Fiber<P, R> {
    pub fn new(fun: fn(P) -> R) -> Self {
        let state = FiberState::new();
        Self {
            fun,
            state,
            is_dirty: false,
        }
    }
}

impl<P, R> FnOnce<(P,)> for Fiber<P, R> {
    type Output = R;
    extern "rust-call" fn call_once(mut self, args: (P,)) -> Self::Output {
        self.state.hook_index = 0;

        // Attach Context to the current FiberState so function can access it
        CURRENT_FIBER_STATE.with(|state| {
            *state.borrow_mut() = Some(&mut self.state as *mut FiberState);
        });

        // Execute the Fiber and get the result
        let result = (self.fun)(args.0);

        // Cleanup the Context
        CURRENT_FIBER_STATE.with(|state| {
            *state.borrow_mut() = None;
        });

        result
    }
}

impl<P, R> FnMut<(P,)> for Fiber<P, R> {
    extern "rust-call" fn call_mut(&mut self, args: (P,)) -> Self::Output {
        self.state.hook_index = 0;

        // Attach Context to the current FiberState so function can access it
        CURRENT_FIBER_STATE.with(|state| {
            *state.borrow_mut() = Some(&mut self.state as *mut FiberState);
        });

        // Execute the Fiber and get the result
        let result = (self.fun)(args.0);

        // Cleanup the Context
        CURRENT_FIBER_STATE.with(|state| {
            *state.borrow_mut() = None;
        });

        result
    }
}

pub(crate) trait ErasedFiber: Any {
    fn state_mut(&mut self) -> &mut FiberState;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn is_dirty(&self) -> bool;
    fn set_dirty(&mut self, is_dirty: bool);
}

impl<P, R> ErasedFiber for Fiber<P, R>
where
    P: 'static,
    R: 'static,
{
    fn state_mut(&mut self) -> &mut FiberState {
        &mut self.state
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn is_dirty(&self) -> bool {
        self.is_dirty
    }
    fn set_dirty(&mut self, is_dirty: bool) {
        self.is_dirty = is_dirty;
    }
}
