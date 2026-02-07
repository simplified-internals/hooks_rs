use hooks_rs::{Fiber, hooks::use_state};

#[test]
fn different_fibers_shouldnt_share_state() {
    fn counter(_: ()) -> i32 {
        let (count, set_count) = use_state(|| 0);
        set_count(&|prev| prev + 1);
        count
    }

    let mut fiber_a = Fiber::new(counter);
    let mut fiber_b = Fiber::new(counter);

    assert_eq!(fiber_a.call(()), 0);
    assert_eq!(fiber_a.call(()), 1);

    assert_eq!(fiber_b.call(()), 0);
    assert_eq!(fiber_b.call(()), 1);

    assert_eq!(fiber_a.call(()), 2);
    assert_eq!(fiber_b.call(()), 2);
}
