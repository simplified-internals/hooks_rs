use std::any::Any;

use crate::fiber::HooksState;

pub(crate) struct Fiber<P, R> {
    pub(crate) fun: fn(P) -> R,
    pub(crate) state: HooksState,
}

impl<P, R> Fiber<P, R> {
    pub(crate) fn new(fun: fn(P) -> R) -> Self {
        let state = HooksState::new();
        Self { fun, state }
    }
    pub(crate) fn call(&mut self, args: P) -> R {
        self.state.hook_index = 0;

        // Execute the Fiber and get the result
        let result = (self.fun)(args);

        result
    }
}

pub(crate) trait ErasedFiber: Any {
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn state_ptr_mut(&mut self) -> *mut HooksState;
    fn state_ptr(&self) -> *const HooksState;
}

impl<P, R> ErasedFiber for Fiber<P, R>
where
    P: 'static,
    R: 'static,
{
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn state_ptr_mut(&mut self) -> *mut HooksState {
        &mut self.state as *mut HooksState
    }

    fn state_ptr(&self) -> *const HooksState {
        &self.state as *const HooksState
    }
}
