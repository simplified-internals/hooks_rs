use hooks_rs::{DynEq, Fiber, use_effect};
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
    fiber(1);
    assert_eq!(CALLS.load(Ordering::Relaxed), 1);

    // Shouldn't run if deps don t change
    fiber(1);
    assert_eq!(CALLS.load(Ordering::Relaxed), 1);

    // Should run if props do change
    fiber(2);
    assert_eq!(CALLS.load(Ordering::Relaxed), 2);

    fiber(3);
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

    fiber(vec![Box::new(MyStruct { x: 1 }), Box::new(1)]);
    assert_eq!(CALLS.load(Ordering::Relaxed), 1);

    // Shouldn't run if same deps
    fiber(vec![Box::new(MyStruct { x: 1 }), Box::new(1)]);
    assert_eq!(CALLS.load(Ordering::Relaxed), 1);

    // Shoul run if deps change
    fiber(vec![Box::new(MyStruct { x: 2 }), Box::new(1)]);
    assert_eq!(CALLS.load(Ordering::Relaxed), 2);

    fiber(vec![Box::new(MyStruct { x: 2 }), Box::new(2)]);
    assert_eq!(CALLS.load(Ordering::Relaxed), 3);
}
