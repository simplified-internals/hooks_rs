mod state;
pub use state::HooksState;

mod node;
pub(crate) use node::*;

mod tree;
pub(crate) use tree::*;

use crate::FiberStoreError;

/// Mount a fiber in the global fiber tree.
pub fn mount_fiber<P, R>(
    parent: Option<String>,
    id: impl Into<String>,
    fun: fn(P) -> R,
) -> Result<(), FiberStoreError>
where
    P: 'static,
    R: 'static,
{
    let id = id.into();
    FIBER_TREE.with(|t| t.borrow_mut().mount_fiber(parent, id, fun))
}

/// Unmount a fiber (and all descendants) from the global fiber tree.
pub fn unmount_fiber(id: impl Into<String>) {
    let id = id.into();
    FIBER_TREE.with(|t| t.borrow_mut().unmount_fiber(id))
}

/// Call a fiber from the global fiber tree.
pub fn call_fiber<P, R>(id: impl Into<String>, props: P) -> Result<R, FiberStoreError>
where
    P: 'static,
    R: 'static,
{
    let id = id.into();

    let fiber_rc = FIBER_TREE.with(|t| {
        let tree = t.borrow();
        let node = tree
            .0
            .get(&id)
            .ok_or_else(|| FiberStoreError::FiberDoesntExist(id.clone()))?;
        Ok(node.fiber.clone())
    })?;

    CURRENT_FIBER_ID.with(|cell| *cell.borrow_mut() = Some(id.clone()));

    let fiber_ptr = {
        let mut fiber_any = fiber_rc.borrow_mut();
        let fiber = fiber_any
            .as_any_mut()
            .downcast_mut::<Fiber<P, R>>()
            .expect("Fiber type mismatch");
        fiber as *mut Fiber<P, R>
    };

    let res = unsafe { (&mut *fiber_ptr).call(props) };

    CURRENT_FIBER_ID.with(|cell| *cell.borrow_mut() = None);

    Ok(res)
}

/// Gets the children ids of a fiber node.
pub fn get_children_ids(id: impl Into<String>) -> Result<Vec<String>, FiberStoreError> {
    let id = id.into();
    FIBER_TREE.with(|t| {
        let tree = t.borrow();
        let node = tree
            .0
            .get(&id)
            .ok_or_else(|| FiberStoreError::FiberDoesntExist(id))?;
        Ok(node.children.clone())
    })
}

/// Gets the parent id of a fiber node.
pub fn get_parent_id(id: impl Into<String>) -> Result<Option<String>, FiberStoreError> {
    let id = id.into();
    FIBER_TREE.with(|t| {
        let tree = t.borrow();
        let node = tree
            .0
            .get(&id)
            .ok_or_else(|| FiberStoreError::FiberDoesntExist(id))?;
        Ok(node.parent.clone())
    })
}
