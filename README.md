# `hooks-rs`

## Contents

- [Problem](#problem)
- [What we want](#what-we-want)
- [Constraints](#constraints)
- [Minimal design](#minimal-design)
  - [1) A per-instance `FiberState`](#1-a-per-instance-fiberstate)
  - [2) A thread-local pointer to the active `FiberState`](#2-a-thread-local-pointer-to-the-active-fiberstate)
  - [3) A tiny `Fiber` wrapper that installs the context](#3-a-tiny-fiber-wrapper-that-installs-the-context)
  - [4) Hook storage as an enum](#4-hook-storage-as-an-enum)
- [`use_ref`: the minimal hook](#use_ref-the-minimal-hook)
  - [Behavior (mount vs update)](#behavior-mount-vs-update)
  - [Implementation](#implementation)
- [How to implement your own hooks (enum style)](#how-to-implement-your-own-hooks-enum-style)
- [Common pitfalls](#common-pitfalls)

## Problem

Rust functions are **stateless**: each call starts fresh unless you explicitly pass state in/out.

Sometimes you want a function to behave like an *instance* that can be “rendered” repeatedly:

- It can be called many times.
- It can keep **private state** between calls.
- The state is attached to an instance, not global.

## What we want

- **Ergonomic** “use_*()” calls inside the function body.
- **Per-instance** storage for hook state.
- A simple model: first call **mounts** hook slots, later calls **update** them and lastyly, dropping an instance should **unmount** the hooks.

## Constraints

The classic “hooks” design is shaped by a few unavoidable constraints:

- **Hooks need a current execution context**.
  - When `use_*()` is called, it must know *which instance* it belongs to.
- **Hook identity is usually by call order**.
  - Without explicit keys, the \(n\)th hook call corresponds to the \(n\)th slot.
  - Therefore hooks **must not be called conditionally**, and order/count must match across calls.
- **Threading**.
  - A global "active context" must be thread-local or synchronized.

## Minimal design

### 1) A per-instance `FiberState`

One instance owns:

- a vector of hook slots
- the current slot index for the in-progress render

```rust
use std::any::Any;

struct FiberState {
    pub hooks: Vec<HookSlot>,
    pub hook_index: usize,
}

impl FiberState {
    pub fn new() -> Self {
        Self { hooks: Vec::new(), hook_index: 0 }
    }
}
```

### 2) A thread-local pointer to the active `FiberState`

Hooks need to find "the current instance". A minimal solution is a thread-local raw pointer:

```rust
use std::cell::RefCell;

thread_local! {
    // Safety:
    // - The fiber installs this pointer only for the duration of `Fiber::call`.
    // - The pointer must remain valid for the entire call (no relocation or drop).
    //
    // If you are sure you want to store fibers in a tree,
    // storing an ID and resolving it through the tree is safer than
    // holding a raw pointer.
    static CURRENT_FIBER_STATE: RefCell<Option<*mut FiberState>> = RefCell::new(None);
}

fn read_current_state() -> &'static mut FiberState {
    CURRENT_FIBER_STATE.with(|f| {
        let fiber_state = unsafe { &mut *f.borrow().expect("hook called outside of a fiber") };
        fiber_state
    })
}
```

### 3) A tiny `Fiber` wrapper that installs the context

This wrapper owns a `FiberState`, resets the index each call, attaches context, calls the function, then cleans up context.

```rust
pub struct Fiber<P, R> {
    fun: fn(P) -> R,
    state: FiberState,
}

impl<P, R> Fiber<P, R> {
    pub fn new(fun: fn(P) -> R) -> Self {
        Self { fun, state: FiberState::new() }
    }

    pub fn call(&mut self, props: P) -> R {
        self.state.hook_index = 0;

        CURRENT_FIBER_STATE.with(|cell| *cell.borrow_mut() = Some(&mut self.state as *mut _));
        let out = (self.fun)(props);
        CURRENT_FIBER_STATE.with(|cell| *cell.borrow_mut() = None);

        out
    }
}
```

### 4) Hook storage as an enum

With the **enum approach**, each hook kind is a variant. This is a **closed set**: to add a new hook kind, you add a new variant and match on it in that hook’s implementation.

For `use_ref`, we only need one variant. We still use type erasure (`Any`) inside the variant so the slot can hold any `T`.

```rust
use std::{any::Any, rc::Rc};

pub enum HookSlot {
    UseRef { current: Rc<dyn Any> },
}
```

If the hook order changes between renders (for example by calling hooks conditionally), the slot at an index will hold a different variant and we should panic with a helpful message.

## `use_ref`: the minimal hook

`use_ref` returns a stable reference-like handle across renders. In Rust, a convenient shape is `Rc<RefCell<T>>`.

### Behavior (mount vs update)

On each render:

- **Mount** (first time this slot index is seen): allocate `Rc<RefCell<T>>`, store it in the slot, return it.
- **Update**: retrieve the stored `Rc`, downcast it back to `Rc<RefCell<T>>`, return it.

There is no special unmount logic required: when the fiber instance is dropped, the `Rc` is dropped as well.

### Implementation

```rust
use std::{any::Any, cell::RefCell, rc::Rc};

pub fn use_ref<T: 'static>(initial: T) -> Rc<RefCell<T>> {
    let fiber_state = read_current_state();

    let idx = fiber_state.hook_index;
    fiber_state.hook_index += 1;

    // Mount
    if idx >= fiber_state.hooks.len() {
        let rc: Rc<RefCell<T>> = Rc::new(RefCell::new(initial));
        fiber_state.hooks.push(HookSlot::UseRef {
            current: rc.clone() as Rc<dyn Any>,
        });
        return rc;
    }

    // Update
    match &fiber_state.hooks[idx] {
        HookSlot::UseRef { current } => current
            .clone()
            .downcast::<RefCell<T>>()
            .expect("hook type mismatch (did hook order change?)"),
        _other => panic!("hook mismatch at slot {idx} (did hook order change?)"),
    }
}
```

## How to implement your own hooks (enum style)

For each new hook kind:

- Add a new `HookSlot::YourHook { ... }` variant.
- Write a `use_your_hook(...) -> ...` function that:
  - reads the current `FiberState`
  - consumes the next `hook_index` slot
  - **mounts** by pushing a `HookSlot::YourHook { ... }` when `idx >= hooks.len()`
  - **updates** by matching `fiber_state.hooks[idx]` and mutating/reading the stored fields
  - panics on mismatch (hook order changed)

### Common pitfalls

- **Conditional hooks**: breaks slot identity (order changes).
- **Non-`'static` state**: type erasure via `Any` typically implies `'static`.
- **Re-entrancy (nested fibers)**: if you call a `Fiber` from inside another `Fiber` call on the same thread, the child overwrites `CURRENT_FIBER_STATE` and clears it when it finishes, leaving the parent fiber without an active context when it resumes.

```rust
let mut parent = Fiber::new(|()| {
    // Slot 0 is fine.
    let a = use_ref(1);

    // Nested call installs a different CURRENT_FIBER_STATE and clears it when it finishes.
    let mut child = Fiber::new(|()| {
        let x = use_ref("child");
    });
    child.call(());

    // This will panic because the parent context is gone.
    let b = use_ref(2);
});

parent.call(());
```


#### Minimal fix (recommended): save/restore the previous CURRENT_FIBER_STATE value

Keep the TLS type as `Option<*mut FiberState>`, but **restore the previous value** when a call ends (instead of always setting it to `None`).

```rust
use std::cell::RefCell;

thread_local! {
    static CURRENT_FIBER_STATE: RefCell<Option<*mut FiberState>> = RefCell::new(None);
}

struct StateGuard(Option<*mut FiberState>);

impl Drop for StateGuard {
    fn drop(&mut self) {
        CURRENT_FIBER_STATE.with(|cell| *cell.borrow_mut() = self.0);
    }
}

fn install_state(ptr: *mut FiberState) -> StateGuard {
    let prev = CURRENT_FIBER_STATE.with(|cell| cell.borrow_mut().replace(Some(ptr)));
    StateGuard(prev)
}

// In Fiber::call (conceptually):
// let _guard = install_state(&mut self.state as *mut _);
// let out = (self.fun)(props);
// drop(_guard) restores the previous pointer automatically (even on panic).
```

This restores correctness of the CURRENT_FIBER_STATE bookkeeping, but it does not make nested fibers a supported or safe abstraction.

Why this is not recommended:

- **Pointer lifetime hazards**: restoring a raw `*mut FiberState` assumes the pointed-to state is still valid.
  If fibers are stored inside a tree or arena that can be reallocated
  (e.g. during `mount_or_call`), previously saved pointers may dangle.

#### Alternative: a stack (only if you need ancestor access)

If you specifically want hooks to be able to inspect *parent* contexts (e.g. for a `use_context` that walks up the current call stack), you can store a stack:

```rust
use std::cell::RefCell;

thread_local! {
    // Safety: the fiber installs this pointer for the duration of a call.
    static CURRENT_FIBER_STATE: RefCell<Vec<*mut FiberState>> = RefCell::new(Vec::new());
}
```

Now:

- `Fiber::call` **pushes** its state pointer before running the function.
- A guard **pops** it when the call ends (even on panic).
- Hooks read the **top** of the stack as "current", and can optionally walk downward to access ancestors.

Why this is not recommended:

- **More pointers**: Same as the other approach.
- **Small overhead**: a borrow + push/pop on every render.