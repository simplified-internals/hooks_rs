pub mod error;
pub mod fiber;
pub mod hooks;
pub mod utils;

pub use fiber::Fiber;

// Utilities

use crate::{error::FiberStoreError, fiber::FiberStore};
use std::cell::RefCell;

thread_local! {
    pub static FIBER_STORE: RefCell<FiberStore> = RefCell::new(FiberStore::new());
}

/// Calls a fiber or initializes it. Fails if there's a fiber at provided `id` that doesn't match `fun`.
pub fn render_fiber<P: 'static, R: 'static>(
    id: u32,
    fun: fn(P) -> R,
    props: P,
) -> Result<R, FiberStoreError> {
    FIBER_STORE.with(|store| {
        let mut store = store.borrow_mut();
        store.mount_or_call(id, fun, props)
    })
}
