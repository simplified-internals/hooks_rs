use std::{cell::RefCell, collections::HashMap};

use crate::{Fiber, error::FiberStoreError, fiber::ErasedFiber};

thread_local! {
    pub(crate) static CURRENT_FIBER_ID: RefCell<Option<u32>> = RefCell::new(None);
    pub static FIBER_TREE: RefCell<FiberTree> = RefCell::new(FiberTree::new());
}

/// A tree of fibers where each node can have children.
pub struct FiberTree(pub(crate) HashMap<u32, FiberNode>);

pub(crate) struct FiberNode {
    pub(crate) fiber: Box<dyn ErasedFiber>,
    pub(crate) parent: Option<u32>,
    pub(crate) children: Vec<u32>,
}

impl FiberTree {
    /// Create a new empty fiber tree node. Assumes the root fiber is at id 0.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Mount a fiber under a parent node.
    pub fn mount_fiber<P, R>(
        &mut self,
        parent: Option<u32>,
        id: u32,
        fun: fn(P) -> R,
    ) -> Result<(), FiberStoreError>
    where
        P: 'static,
        R: 'static,
    {
        if self.0.contains_key(&id) {
            return Err(FiberStoreError::FiberAlreadyExists(id));
        }

        // Insert the fiber into the nodes map
        self.0.insert(
            id,
            FiberNode {
                fiber: Box::new(Fiber::new(fun)),
                parent,
                children: Vec::new(),
            },
        );

        if let Some(parent) = parent {
            if let Some(parent) = self.0.get_mut(&parent) {
                parent.children.push(id);
            } else {
                return Err(FiberStoreError::ParentDoesNotExist(parent));
            }
        }

        Ok(())
    }

    /// Unmount a fiber and all its descendants
    pub fn unmount_fiber(&mut self, id: u32) {
        if let Some(node) = self.0.remove(&id) {
            for child in node.children {
                self.unmount_fiber(child);
            }

            if let Some(parent) = node.parent {
                if let Some(parent) = self.0.get_mut(&parent) {
                    parent.children.retain(|&c| c != id);
                }
            }
        }
    }

    /// Call an existing fiber.
    pub fn call<P: 'static, R: 'static>(
        &mut self,
        id: u32,
        props: P,
    ) -> Result<R, FiberStoreError> {
        let node = self
            .0
            .get_mut(&id)
            .ok_or_else(|| FiberStoreError::FiberDoesntExist(id))?;

        let fiber = node
            .fiber
            .as_any_mut()
            .downcast_mut::<Fiber<P, R>>()
            .expect("Fiber type mismatch");

        CURRENT_FIBER_ID.with(|id_ref| {
            *id_ref.borrow_mut() = Some(id);
        });

        let res = fiber(props);

        CURRENT_FIBER_ID.with(|id_ref| {
            *id_ref.borrow_mut() = None;
        });

        Ok(res)
    }
}
