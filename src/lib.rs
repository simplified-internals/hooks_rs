#![feature(unboxed_closures, fn_traits, core_intrinsics)]
// modules
mod error;
mod fiber;
mod hooks;
mod utils;

// ------------------------------------ API surface ------------------------------------

// ----------------- Errors
pub use error::FiberStoreError;

// ----------------- Fiber Management
pub use fiber::{call_fiber, get_children_ids, get_parent_id, mount_fiber, unmount_fiber};

// ----------------- Hooks

// --- Hook Creation
pub use hooks::{Hook, read_fiber_state};
pub use fiber::HooksState;

// --- Default hooks
pub use hooks::use_context::{Context, create_context, provide_context, use_context};
pub use hooks::use_effect::use_effect;
pub use utils::DynEq;
pub use hooks::use_ref::use_ref;
pub use hooks::use_state::{SetStateAction, use_state};
