use hooks_rs::{Fiber, hooks::use_effect};
use std::sync::atomic::{AtomicU64, Ordering};

#[test]
fn run_on_dep_change() {
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

    fiber.call(1);
    assert_eq!(CALLS.load(Ordering::Relaxed), 1);

    fiber.call(1);
    assert_eq!(CALLS.load(Ordering::Relaxed), 1);

    fiber.call(2);
    assert_eq!(CALLS.load(Ordering::Relaxed), 2);

    fiber.call(3);
    assert_eq!(CALLS.load(Ordering::Relaxed), 3);
}

#[test]
fn deps_are_custom() {
    static CALLS: AtomicU64 = AtomicU64::new(0);

    #[derive(PartialEq, Debug)]
    struct MyStruct {
        x: i32,
    }

    fn component(dep: MyStruct) {
        use_effect(
            &mut || {
                CALLS.fetch_add(1, Ordering::Relaxed);
            },
            vec![Box::new(dep)],
        );
    }

    let mut fiber = Fiber::new(|dep| component(dep));

    fiber.call(MyStruct { x: 1 });
    assert_eq!(CALLS.load(Ordering::Relaxed), 1);

    fiber.call(MyStruct { x: 1 }); // same → no run
    assert_eq!(CALLS.load(Ordering::Relaxed), 1);

    fiber.call(MyStruct { x: 2 }); // changed → runs
    assert_eq!(CALLS.load(Ordering::Relaxed), 2);
}
