use std::{any::Any, cell::RefCell, collections::HashMap, hash::Hash};

use crate::{error::FiberStoreError, hooks::Hooks};

thread_local! {
    pub(crate) static CURRENT_FIBER: RefCell<Option<*mut FiberState>> = RefCell::new(None);
}

pub(crate) struct FiberState {
    pub(crate) hooks: Vec<Hooks>,
    pub(crate) hook_index: usize,
}

impl FiberState {
    pub(crate) fn new() -> Self {
        Self {
            hooks: Vec::new(),
            hook_index: 0,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.hook_index = 0;
    }
}

pub struct Fiber<P, R> {
    fun: fn(P) -> R,
    core: FiberState,
}

impl<P, R> Fiber<P, R> {
    pub fn new(fun: fn(P) -> R) -> Self {
        Self {
            fun,
            core: FiberState::new(),
        }
    }

    pub fn call(&mut self, props: P) -> R {
        self.core.reset();

        CURRENT_FIBER.with(|f| {
            *f.borrow_mut() = Some(&mut self.core as *mut FiberState);
        });

        let ret = (self.fun)(props);

        CURRENT_FIBER.with(|f| {
            *f.borrow_mut() = None;
        });

        ret
    }
}

pub struct FiberStore {
    fibers: HashMap<u32, Box<dyn Any>>,
}

impl FiberStore {
    /// Creates a new fiber store
    pub fn new() -> Self {
        Self {
            fibers: HashMap::new(),
        }
    }
    /// Mount a new fiber. Fails if one already exists at this ID.
    pub fn mount<P: 'static, R: 'static>(
        &mut self,
        id: u32,
        fun: fn(P) -> R,
    ) -> Result<(), FiberStoreError> {
        if self.fibers.contains_key(&id) {
            return Err(FiberStoreError::FiberAlreadyExists(id));
        }
        self.fibers.insert(
            id,
            Box::new(Fiber {
                fun,
                core: FiberState::new(),
            }),
        );
        Ok(())
    }

    /// Call an existing fiber. Fails if the fiber doesn't exist or type doen't match.
    pub fn call<P: 'static, R: 'static>(
        &mut self,
        id: u32,
        props: P,
    ) -> Result<R, FiberStoreError> {
        let fiber = self
            .fibers
            .get_mut(&id)
            .ok_or_else(|| FiberStoreError::FiberDoesntExist(id))?;

        let fiber = fiber
            .as_mut()
            .downcast_mut::<Fiber<P, R>>()
            .ok_or_else(|| FiberStoreError::FiberTypeMismatch(id))?;

        Ok(fiber.call(props))
    }

    /// Mount if missing, otherwise call existing fiber. Fails if type mismatches.
    pub fn mount_or_call<P: 'static, R: 'static>(
        &mut self,
        id: u32,
        fun: fn(P) -> R,
        props: P,
    ) -> Result<R, FiberStoreError> {
        if self.fibers.contains_key(&id) {
            self.call(id, props)
        } else {
            self.mount(id, fun)?;
            self.call(id, props)
        }
    }

    /// Unmount a fiber.
    pub fn unmount(&mut self, id: u32) {
        self.fibers.remove(&id);
    }
}
