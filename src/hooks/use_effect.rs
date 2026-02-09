use crate::{
    hooks::{Hooks, read_fiber_state},
    utils::{DynEq, deps_changed},
};

#[track_caller]
pub fn use_effect(effect: &mut impl FnMut(), deps: Vec<Box<dyn DynEq>>) {
    let location = std::panic::Location::caller();

    let fiber_state = read_fiber_state(&format!(
        "Hook `use_effect` was called outside of a fiber. ({})",
        location
    ));

    let idx = fiber_state.hook_index;
    fiber_state.hook_index += 1;

    if idx >= fiber_state.hooks.len() {
        // MOUNT LOGIC HERE
        fiber_state.hooks.push(Hooks::UseEffect { deps });
        effect();
        return;
    }

    // UPDATE LOGIC HERE
    let prev_deps = match &mut fiber_state.hooks[idx] {
        Hooks::UseEffect { deps: prev_deps } => prev_deps,
        other => panic!(
            "Expected `use_hook` hook, but got `{other}`. This may happen when calling hooks conditionally. ({})",
            location
        ),
    };

    if deps_changed(prev_deps, &deps) {
        effect();
        *prev_deps = deps;
    }
}
