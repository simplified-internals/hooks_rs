use std::cell::RefCell;

use crate::hooks::Hooks;

thread_local! {
    pub(crate) static CURRENT_FIBER: RefCell<Option<*mut FiberState>> = RefCell::new(None);
}

pub(crate) struct FiberState {
    pub(crate) hooks: Vec<Hooks>,
    pub(crate) hook_index: usize,
}

impl FiberState {
    pub(crate) fn new() -> Self {
        Self {
            hooks: Vec::new(),
            hook_index: 0,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.hook_index = 0;
    }
}

pub struct Fiber<P, R> {
    fun: fn(P) -> R,
    core: FiberState,
}

impl<P, R> Fiber<P, R> {
    pub fn new(fun: fn(P) -> R) -> Self {
        Self {
            fun,
            core: FiberState::new(),
        }
    }

    pub fn call(&mut self, props: P) -> R {
        self.core.reset();

        CURRENT_FIBER.with(|f| {
            *f.borrow_mut() = Some(&mut self.core as *mut FiberState);
        });

        let ret = (self.fun)(props);

        CURRENT_FIBER.with(|f| {
            *f.borrow_mut() = None;
        });

        ret
    }
}
