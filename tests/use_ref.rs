use std::{cell::RefCell, rc::Rc};

use hooks_rs::{Fiber, use_ref};

#[test]
fn should_work_single() {
    #[derive(Debug, PartialEq)]
    struct MyStruct {
        x: i32,
    }

    fn component(_: ()) -> Rc<RefCell<MyStruct>> {
        use_ref(MyStruct { x: 5 })
    }

    let mut fiber = Fiber::new(component);

    let r1 = fiber(());
    assert_eq!(r1.borrow().x, 5);

    r1.borrow_mut().x = 20;
    let r2 = fiber(());
    assert_eq!(r2.borrow().x, 20);
}

#[test]
fn multiple_refs_in_same_component() {
    fn component(_: ()) -> (Rc<RefCell<i32>>, Rc<RefCell<i32>>) {
        let a = use_ref(1);
        let b = use_ref(2);
        (a, b)
    }

    let mut fiber = Fiber::new(component);

    let (a1, b1) = fiber(());
    *a1.borrow_mut() = 10;
    *b1.borrow_mut() = 20;

    let (a2, b2) = fiber(());
    assert_eq!(*a2.borrow(), 10);
    assert_eq!(*b2.borrow(), 20);
}
