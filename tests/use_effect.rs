use hooks_rs::{Fiber, hooks::use_effect, utils::DynEq};
use std::sync::atomic::{AtomicU64, Ordering};

#[test]
fn should_work_single() {
    static CALLS: AtomicU64 = AtomicU64::new(0);

    fn component(dep: i32) -> () {
        use_effect(
            &mut || {
                CALLS.fetch_add(1, Ordering::Relaxed);
            },
            vec![Box::new(dep)],
        );
    }

    let mut fiber = Fiber::new(|dep| component(dep));

    // Should run on mount
    fiber.call(1);
    assert_eq!(CALLS.load(Ordering::Relaxed), 1);

    // Shouldn't run if deps don t change
    fiber.call(1);
    assert_eq!(CALLS.load(Ordering::Relaxed), 1);

    // Should run if props do change
    fiber.call(2);
    assert_eq!(CALLS.load(Ordering::Relaxed), 2);

    fiber.call(3);
    assert_eq!(CALLS.load(Ordering::Relaxed), 3);
}

#[test]
fn any_deps_should_work() {
    static CALLS: AtomicU64 = AtomicU64::new(0);

    #[derive(PartialEq, Debug)]
    struct MyStruct {
        x: i32,
    }

    fn component(deps: Vec<Box<dyn DynEq>>) {
        use_effect(
            &mut || {
                CALLS.fetch_add(1, Ordering::Relaxed);
            },
            deps,
        );
    }

    let mut fiber = Fiber::new(|deps| component(deps));

    fiber.call(vec![Box::new(MyStruct { x: 1 }), Box::new(1)]);
    assert_eq!(CALLS.load(Ordering::Relaxed), 1);

    // Shouldn't run if same deps
    fiber.call(vec![Box::new(MyStruct { x: 1 }), Box::new(1)]);
    assert_eq!(CALLS.load(Ordering::Relaxed), 1);

    // Shoul run if deps change
    fiber.call(vec![Box::new(MyStruct { x: 2 }), Box::new(1)]);
    assert_eq!(CALLS.load(Ordering::Relaxed), 2);

    fiber.call(vec![Box::new(MyStruct { x: 2 }), Box::new(2)]);
    assert_eq!(CALLS.load(Ordering::Relaxed), 3);
}
