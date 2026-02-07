use std::sync::atomic::{AtomicU64, Ordering};

use hooks_rs::{Fiber, hooks::use_state};

#[test]
fn works() {
    fn counter(_: ()) -> i32 {
        let (count, set_count) = use_state(|| 0);
        set_count(count + 1);
        count
    }

    let mut fiber = Fiber::new(counter);

    assert_eq!(fiber.call(()), 0);
    assert_eq!(fiber.call(()), 1);
    assert_eq!(fiber.call(()), 2);
    assert_eq!(fiber.call(()), 3);
}

#[test]
fn preserves_order() {
    fn component(_: ()) -> (i32, i32) {
        let (a, set_a) = use_state(|| 1);
        let (b, set_b) = use_state(|| 10);

        set_a(a + 1);
        set_b(b + 10);

        (a, b)
    }

    let mut fiber = Fiber::new(component);

    assert_eq!(fiber.call(()), (1, 10));
    assert_eq!(fiber.call(()), (2, 20));
    assert_eq!(fiber.call(()), (3, 30));
}

#[test]
fn initial_state_is_only_called_once() {
    static CALLS: AtomicU64 = AtomicU64::new(0);

    fn component(_: ()) -> i32 {
        let (value, _) = use_state(|| {
            CALLS.fetch_add(1, Ordering::Relaxed);

            42
        });
        value
    }

    let mut fiber = Fiber::new(component);

    assert_eq!(fiber.call(()), 42);
    assert_eq!(fiber.call(()), 42);
    assert_eq!(fiber.call(()), 42);

    assert_eq!(CALLS.load(Ordering::Relaxed), 1);
}

#[test]
fn heterogeneous_hook_types_work() {
    fn component(_: ()) -> (i32, String) {
        let (count, set_count) = use_state(|| 0);
        let (text, set_text) = use_state(|| String::from("hi"));

        set_count(count + 1);
        set_text(format!("{text}!"));

        (count, text)
    }

    let mut fiber = Fiber::new(component);

    assert_eq!(fiber.call(()), (0, "hi".into()));
    assert_eq!(fiber.call(()), (1, "hi!".into()));
    assert_eq!(fiber.call(()), (2, "hi!!".into()));
}

#[test]
#[should_panic]
fn use_state_outside_fiber_panics() {
    let _ = use_state(|| 0);
}
