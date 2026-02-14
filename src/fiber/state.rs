use crate::hooks::Hook;

pub struct HooksState {
    pub hooks: Vec<Hook>,
    pub hook_index: usize,
}

impl HooksState {
    pub(crate) fn new() -> Self {
        Self {
            hooks: Vec::new(),
            hook_index: 0,
        }
    }
}
