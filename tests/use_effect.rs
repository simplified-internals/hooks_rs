use hooks_rs::{DynEq, call_fiber, mount_fiber, use_effect};
use std::sync::atomic::{AtomicU64, Ordering};

#[test]
fn should_work_single() {
    static CALLS: AtomicU64 = AtomicU64::new(0);

    fn component(dep: i32) {
        use_effect(
            &mut || {
                CALLS.fetch_add(1, Ordering::Relaxed);
            },
            vec![Box::new(dep)],
        );
    }

    mount_fiber(None, "root", |dep| component(dep)).unwrap();

    // Should run on mount
    let component = |deps| call_fiber::<i32, ()>("root", deps).unwrap();

    component(1);
    assert_eq!(CALLS.load(Ordering::Relaxed), 1);

    // Shouldn't run if deps don t change
    component(1);
    assert_eq!(CALLS.load(Ordering::Relaxed), 1);

    // Should run if props do change
    component(2);
    assert_eq!(CALLS.load(Ordering::Relaxed), 2);

    component(3);
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

    mount_fiber(None, "root", |deps| component(deps)).unwrap();

    let component = |deps| call_fiber::<Vec<Box<dyn DynEq>>, ()>("root", deps).unwrap();

    component(vec![Box::new(MyStruct { x: 1 }), Box::new(1)]);
    assert_eq!(CALLS.load(Ordering::Relaxed), 1);

    // Shouldn't run if same deps
    component(vec![Box::new(MyStruct { x: 1 }), Box::new(1)]);
    assert_eq!(CALLS.load(Ordering::Relaxed), 1);

    // Shoul run if deps change
    component(vec![Box::new(MyStruct { x: 2 }), Box::new(1)]);
    assert_eq!(CALLS.load(Ordering::Relaxed), 2);

    component(vec![Box::new(MyStruct { x: 2 }), Box::new(2)]);
    assert_eq!(CALLS.load(Ordering::Relaxed), 3);
}
