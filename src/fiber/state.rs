use crate::hooks::Hook;

pub struct FiberState {
    pub hooks: Vec<Hook>,
    pub hook_index: usize,
}

impl FiberState {
    pub(crate) fn new() -> Self {
        Self {
            hooks: Vec::new(),
            hook_index: 0,
        }
    }
}
