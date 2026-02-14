use std::sync::atomic::{AtomicU64, Ordering};

use hooks_rs::{call_fiber, mount_fiber, use_state};

#[test]
fn should_work_single() {
    fn counter(_: ()) -> i32 {
        let (count, set_count) = use_state(|| 0);
        set_count(|prev| prev + 1);
        count
    }

    mount_fiber(None, "root", counter).unwrap();

    let component = || call_fiber::<(), i32>("root", ()).unwrap();

    assert_eq!(component(), 0);
    assert_eq!(component(), 1);
    assert_eq!(component(), 2);
    assert_eq!(component(), 3);
}

#[test]
fn should_work_multiple() {
    #[derive(Clone, Debug, PartialEq)]
    pub struct MyNumber(u32);

    fn component(_: ()) -> (MyNumber, String) {
        let (count, set_count) = use_state(|| MyNumber(0));
        let (text, set_text) = use_state(|| String::from("hi"));

        set_count(|prev| MyNumber(prev.0 + 1));
        set_text(|prev_text| format!("{prev_text}!"));

        (count, text)
    }

    mount_fiber(None, "root", component).unwrap();

    let component = || call_fiber::<(), (MyNumber, String)>("root", ()).unwrap();

    assert_eq!(component(), (MyNumber(0), "hi".into()));
    assert_eq!(component(), (MyNumber(1), "hi!".into()));
    assert_eq!(component(), (MyNumber(2), "hi!!".into()));
}

#[test]
fn initial_should_be_called_once() {
    static CALLS: AtomicU64 = AtomicU64::new(0);

    fn component(_: ()) -> i32 {
        let (value, _) = use_state(|| {
            CALLS.fetch_add(1, Ordering::Relaxed);

            42
        });
        value
    }

    mount_fiber(None, "root", component).unwrap();

    let component = || call_fiber::<(), i32>("root", ()).unwrap();

    assert_eq!(component(), 42);
    assert_eq!(component(), 42);

    assert_eq!(CALLS.load(Ordering::Relaxed), 1);
}

#[test]
#[should_panic]
fn usage_outside_fiber_causes_panic() {
    let _ = use_state(|| 0);
}
