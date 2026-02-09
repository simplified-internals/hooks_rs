use crate::{
    fiber::FiberState,
    hooks::{Hooks, read_fiber_state},
};

/// Declares a stateful value that persists across renders.
///
/// This hook returns the current state value and a setter function that can be
/// used to schedule state updates. The state is associated with the current
/// fiber and is identified by call order, so hooks **must not be called
/// conditionally**.
///
/// The `initial` initializer is evaluated **only on the first render**
/// (mount). On subsequent renders, the previously stored state is returned.
///
/// # Type Parameters
///
/// - `S`: The state type. It must be `'static` because it is type-erased
///   internally, and `Clone` because the current state value is returned
///   by value on each render.
///
/// # Panics
///
/// Panics if:
/// - The hook is called outside of a fiber context.
/// - The hook call order changes between renders (e.g. calling hooks
///   conditionally), causing a type mismatch.
///
/// # Behavior
///
/// - **Mount phase**: initializes state using `initial` and stores it
///   in the fiber's hook list.
/// - **Update phase**: retrieves the previously stored state for the
///   current hook index.
///
/// The returned setter is stable across renders and may be cloned and
/// called multiple times. Calling the setter replaces the stored state
/// with a new value derived from the previous one.
///
/// # Examples
///
/// ```rust
/// let (count, set_count) = use_state(|| 0);
///
/// set_count(|prev| prev + 1);
/// ```
#[track_caller]
pub fn use_state<S>(initial: impl FnOnce() -> S) -> (S, SetStateAction<S>)
where
    S: 'static + Clone,
{
    let location = std::panic::Location::caller();

    let fiber_state = read_fiber_state(&format!(
        "Hook `use_state` was called outside of a fiber. ({})",
        location
    ));

    let idx = fiber_state.hook_index;
    fiber_state.hook_index += 1;

    if idx >= fiber_state.hooks.len() {
        // MOUNT LOGIC HERE
        fiber_state.hooks.push(Hooks::UseState {
            value: Box::new(initial()),
        });
    }

    // UPDATE LOGIC HERE
    let state = {
        match &fiber_state.hooks[idx] {
            Hooks::UseState { value } => value.downcast_ref::<S>().unwrap().clone(),
            other => panic!(
                "Expected `use_state` hook, but got `{other}`. This may happen when calling hooks conditionally. ({})",
                location
            ),
        }
    };

    let setter = SetStateAction::<S> {
        fiber_ptr: &mut *fiber_state,
        hook_index: idx,
        _marker: std::marker::PhantomData,
    };

    (state, setter)
}

/// --------------------------- React.Dispatch<SetStateAction<T>> from wish

pub struct SetStateAction<S> {
    fiber_ptr: *mut FiberState,
    hook_index: usize,
    _marker: std::marker::PhantomData<S>,
}

impl<S: Clone + 'static> SetStateAction<S> {
    fn set(&self, f: &dyn Fn(&S) -> S) {
        unsafe {
            let fiber = &mut *self.fiber_ptr;
            let Hooks::UseState { value } = &mut fiber.hooks[self.hook_index] else {
                unreachable!()
            };
            let current = value.downcast_ref::<S>().unwrap();
            *value = Box::new(f(current));
        }
    }
}

/// --------------------------- Clone / Copy to mimick react like usage
// Manually implement these traits since deriving them also makes S: Clone / S: Copy
impl<S> Clone for SetStateAction<S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<S> Copy for SetStateAction<S> {}

// --------------------------- Fn Traits so SetStateAction can be used like a closure
impl<S: Clone + 'static> FnOnce<(&dyn Fn(&S) -> S,)> for SetStateAction<S> {
    type Output = ();

    extern "rust-call" fn call_once(self, args: (&dyn Fn(&S) -> S,)) -> Self::Output {
        self.set(args.0)
    }
}

impl<S: Clone + 'static> FnMut<(&dyn Fn(&S) -> S,)> for SetStateAction<S> {
    extern "rust-call" fn call_mut(&mut self, args: (&dyn Fn(&S) -> S,)) -> Self::Output {
        self.set(args.0)
    }
}

impl<S: Clone + 'static> Fn<(&dyn Fn(&S) -> S,)> for SetStateAction<S> {
    extern "rust-call" fn call(&self, args: (&dyn Fn(&S) -> S,)) -> Self::Output {
        self.set(args.0)
    }
}
