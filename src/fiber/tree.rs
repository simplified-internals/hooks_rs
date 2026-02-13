use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    error::FiberStoreError,
    fiber::{ErasedFiber, Fiber},
};

thread_local! {
    pub static FIBER_TREE: RefCell<FiberTree> = RefCell::new(FiberTree::new());
    pub(crate) static CURRENT_FIBER_ID: RefCell<Option<String>> = RefCell::new(None);
}

/// A tree of fibers where each node can have children.
pub struct FiberTree(pub(crate) HashMap<String, FiberNode>);

pub(crate) struct FiberNode {
    pub(crate) fiber: Rc<RefCell<Box<dyn ErasedFiber>>>,
    pub(crate) parent: Option<String>,
    pub(crate) children: Vec<String>,
}

impl FiberTree {
    /// Create a new empty fiber tree node.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Mount a fiber under a parent node.
    pub fn mount_fiber<P, R>(
        &mut self,
        parent: Option<String>,
        id: String,
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
            id.clone(),
            FiberNode {
                fiber: Rc::new(RefCell::new(Box::new(Fiber::new(fun)))),
                parent: parent.clone(),
                children: Vec::new(),
            },
        );

        if let Some(parent) = parent {
            if let Some(parent) = self.0.get_mut(&parent.to_string()) {
                parent.children.push(id);
            } else {
                return Err(FiberStoreError::ParentDoesNotExist(parent.to_string()));
            }
        }

        Ok(())
    }

    /// Unmount a fiber and all its descendants
    pub fn unmount_fiber(&mut self, id: String) {
        if let Some(node) = self.0.remove(&id) {
            for child in node.children {
                self.unmount_fiber(child);
            }

            if let Some(parent) = node.parent {
                if let Some(parent) = self.0.get_mut(&parent) {
                    parent.children.retain(|c| c != &id);
                }
            }
        }
    }
}
