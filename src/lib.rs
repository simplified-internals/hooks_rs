#![feature(unboxed_closures, fn_traits, core_intrinsics)]
// modules
mod error;
mod fiber;
mod hooks;
mod utils;

// API surface
pub use error::FiberStoreError;

pub use fiber::FiberState;
pub use fiber::{call_fiber, get_children_ids, get_parent_id, mount_fiber, unmount_fiber};

// Utils for letting users create their own hooks;
pub use hooks::{Hook, read_fiber_state};
// ------ Default hooks
pub use hooks::use_context::{Context, create_context, provide_context, use_context};
pub use hooks::use_effect::use_effect;
pub use hooks::use_ref::use_ref;
pub use hooks::use_state::{SetStateAction, use_state};

pub use utils::DynEq;
