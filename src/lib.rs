#![feature(unboxed_closures, fn_traits)]

// Public modules
pub mod error;

// Internal modules
mod fiber;
mod hooks;
mod utils;

// Public, curated API surface
pub use error::FiberStoreError;

pub use fiber::{Fiber, FiberTree, mount_fiber, render_fiber, unmount_fiber};

pub use hooks::{SetStateAction, use_effect, use_ref, use_state};

pub use utils::DynEq;
