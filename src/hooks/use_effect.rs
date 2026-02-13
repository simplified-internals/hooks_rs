use std::{any::TypeId, intrinsics::caller_location};

use crate::{
    hooks::{Hook, read_fiber_state},
    utils::{DynEq, deps_changed},
};

pub(crate) struct UseEffect {
    deps: Vec<Box<dyn DynEq>>,
}

#[track_caller]
pub fn use_effect(effect: &mut impl FnMut(), deps: Vec<Box<dyn DynEq>>) {
    let location = caller_location();

    let fiber_state = read_fiber_state(&format!(
        "Hook `use_effect` was called outside of a fiber. ({})",
        location
    ));

    let idx = fiber_state.hook_index;
    fiber_state.hook_index += 1;

    if idx >= fiber_state.hooks.len() {
        // MOUNT LOGIC HERE
        fiber_state.hooks.push(Hook {
            type_id: TypeId::of::<UseEffect>(),
            state: Box::new(UseEffect { deps }),
        });
        effect();
        return;
    }

    // UPDATE LOGIC HERE
    let hook = &mut fiber_state.hooks[idx];
    if hook.type_id != TypeId::of::<UseEffect>() {
        panic!("Expected `use_effect` hook, but got `{:?}`.", hook.type_id);
    }
    let use_effect = hook.state.downcast_mut::<UseEffect>().unwrap();
    let prev_deps = &use_effect.deps;

    if deps_changed(prev_deps, &deps) {
        effect();
        use_effect.deps = deps;
    }
}
