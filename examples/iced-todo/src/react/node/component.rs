use hooks_rs::{FiberStoreError, call_fiber, mount_fiber};

use crate::react::VNode;

pub struct ComponentNode<Message: 'static> {
    pub(crate) key: String,
    pub(crate) inner: Box<dyn DynComponent<Message>>,
}

pub(crate) trait DynComponent<Message: 'static> {
    fn mount(&self, parent: &str, id: &str) -> Result<(), FiberStoreError>;
    fn call(self: Box<Self>, id: String) -> Result<VNode<Message>, FiberStoreError>;
}

pub(crate) struct TypedComponent<P, Message: 'static> {
    pub(crate) fun: fn(P) -> VNode<Message>,
    pub(crate) props: P,
}

impl<P, Message> DynComponent<Message> for TypedComponent<P, Message>
where
    P: 'static,
    Message: 'static,
{
    fn mount(&self, parent: &str, id: &str) -> Result<(), FiberStoreError> {
        match mount_fiber(Some(parent.to_string()), id.to_string(), self.fun) {
            Ok(()) => Ok(()),
            Err(FiberStoreError::FiberAlreadyExists(_)) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn call(self: Box<Self>, id: String) -> Result<VNode<Message>, FiberStoreError> {
        call_fiber(id, self.props)
    }
}
