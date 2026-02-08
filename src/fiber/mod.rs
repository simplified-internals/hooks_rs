mod node;
mod state;
mod tree;

pub(crate) use node::ErasedFiber;
pub use node::Fiber;
pub(crate) use state::{CURRENT_FIBER_STATE, FiberState};

pub use tree::FiberTree;

use crate::FiberStoreError;

/// Mount a fiber in the global fiber tree.
pub fn mount_fiber<P, R>(
    parent: Option<u32>,
    id: u32,
    fun: fn(P) -> R,
) -> Result<(), FiberStoreError>
where
    P: 'static,
    R: 'static,
{
    tree::FIBER_TREE.with(|t| t.borrow_mut().mount_fiber(parent, id, fun))
}

/// Unmount a fiber (and all descendants) from the global fiber tree.
pub fn unmount_fiber(id: u32) {
    tree::FIBER_TREE.with(|t| t.borrow_mut().unmount_fiber(id))
}

/// Render a fiber from the global fiber tree, mounting it on first use.
///
/// This is a convenience wrapper around [`FiberTree`] that makes it easy to
/// render component-like functions without manually managing a tree instance.
pub fn render_fiber<P, R>(id: u32, fun: fn(P) -> R, props: P) -> Result<R, FiberStoreError>
where
    P: 'static,
    R: 'static,
{
    let fiber_ptr = tree::FIBER_TREE.with(|t| {
        let mut tree = t.borrow_mut();

        if !tree.0.contains_key(&id) {
            tree.mount_fiber(None, id, fun)?;
        }

        let fiber = tree
            .0
            .get_mut(&id)
            .unwrap()
            .fiber
            .as_any_mut()
            .downcast_mut::<Fiber<P, R>>()
            .unwrap();
        Ok(fiber as *mut Fiber<P, R>)
    })?;

    Ok(unsafe { (&mut *fiber_ptr)(props) })
}
