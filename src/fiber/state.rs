use crate::hooks::Hooks;
use std::cell::RefCell;

thread_local! {
    pub(crate) static CURRENT_FIBER_STATE: RefCell<Option<*mut FiberState>> = RefCell::new(None);
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
}
