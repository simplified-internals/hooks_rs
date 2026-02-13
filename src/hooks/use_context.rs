use std::{
    any::TypeId,
    intrinsics::caller_location,
    marker::PhantomData,
    sync::atomic::{AtomicU64, Ordering},
};

use crate::{
    fiber::{CURRENT_FIBER_ID, FIBER_TREE, get_parent_id},
    hooks::{Hook, read_fiber_state},
};

static NEXT_CONTEXT_ID: AtomicU64 = AtomicU64::new(1);

pub struct Context<T> {
    id: u64,
    _marker: PhantomData<T>,
}
impl<T> Clone for Context<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            _marker: PhantomData,
        }
    }
}
impl<T> Copy for Context<T> {}

pub fn create_context<T>() -> Context<T>
where
    T: 'static,
{
    Context {
        id: NEXT_CONTEXT_ID.fetch_add(1, Ordering::Relaxed),
        _marker: PhantomData,
    }
}

pub(crate) struct ProvidedContext<T>
where
    T: 'static + Clone,
{
    ctx_id: u64,
    value: T,
}

/// Provide a context value for descendants.
///
/// Call this inside a "Provider component" before rendering children.
pub fn provide_context<T>(ctx: Context<T>, value: T)
where
    T: 'static + Clone,
{
    let fiber_state = read_fiber_state("`provide_context` was called outside of a fiber");

    let idx = fiber_state.hook_index;
    fiber_state.hook_index += 1;

    if idx >= fiber_state.hooks.len() {
        fiber_state.hooks.push(Hook {
            type_id: TypeId::of::<ProvidedContext<T>>(),
            state: Box::new(ProvidedContext {
                ctx_id: ctx.id,
                value,
            }),
        });
        return;
    }

    let hook = &mut fiber_state.hooks[idx];
    if hook.type_id != TypeId::of::<ProvidedContext<T>>() {
        panic!(
            "Expected `provide_context` hook, but got `{:?}`.",
            hook.type_id
        );
    }

    let provided = hook
        .state
        .downcast_mut::<ProvidedContext<T>>()
        .expect("type checked above");

    if provided.ctx_id != ctx.id {
        panic!("Context mismatch: `provide_context` call order changed.");
    }

    provided.value = value;
}

/// Read the nearest provided context value by walking up the active fiber stack.
///
/// If no provider is found, returns the context's default value.
#[track_caller]
pub fn use_context<T>(ctx: Context<T>) -> T
where
    T: 'static + Clone,
{
    let location = caller_location();

    let mut current_id = CURRENT_FIBER_ID.with(|cell| cell.borrow().clone());
    if current_id.is_none() {
        panic!("`use_context` was called outside of a fiber ({})", location);
    }

    while let Some(id) = current_id {
        // Borrow the tree briefly to clone the fiber handle.
        let fiber_rc = FIBER_TREE.with(|t| {
            let tree = t.borrow();
            let node = tree
                .0
                .get(&id)
                .unwrap_or_else(|| panic!("Fiber `{id}` does not exist"));
            node.fiber.clone()
        });

        // Inspect hooks in that fiber's state.
        let found = {
            let fiber_any = fiber_rc.borrow();
            let state_ptr = fiber_any.state_ptr();
            let state = unsafe { &*state_ptr };

            state
                .hooks
                .iter()
                .rev()
                .filter(|h| h.type_id == TypeId::of::<ProvidedContext<T>>())
                .find_map(|h| {
                    let provided = h
                        .state
                        .downcast_ref::<ProvidedContext<T>>()
                        .expect("type checked above");
                    (provided.ctx_id == ctx.id).then(|| provided.value.clone())
                })
        };

        if let Some(v) = found {
            return v;
        }

        current_id = get_parent_id(id).ok().flatten();
    }

    panic!("No context value found for context ({})", location);
}
