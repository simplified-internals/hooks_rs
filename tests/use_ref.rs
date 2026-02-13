use std::{cell::RefCell, rc::Rc};

use hooks_rs::{call_fiber, mount_fiber, use_ref};

#[test]
fn should_work_single() {
    #[derive(Debug, PartialEq)]
    struct MyStruct {
        x: i32,
    }

    fn component(_: ()) -> Rc<RefCell<MyStruct>> {
        use_ref(MyStruct { x: 5 })
    }

    mount_fiber(None, "root", component).unwrap();

    let component = || call_fiber::<(), Rc<RefCell<MyStruct>>>("root", ()).unwrap();

    let r1 = component();
    assert_eq!(r1.borrow().x, 5);

    r1.borrow_mut().x = 20;
    let r2 = component();
    assert_eq!(r2.borrow().x, 20);
}

#[test]
fn multiple_refs_in_same_component() {
    fn component(_: ()) -> (Rc<RefCell<i32>>, Rc<RefCell<i32>>) {
        let a = use_ref(1);
        let b = use_ref(2);
        (a, b)
    }

    mount_fiber(None, "root", component).unwrap();

    let component = || call_fiber::<(), (Rc<RefCell<i32>>, Rc<RefCell<i32>>)>("root", ()).unwrap();

    let (a1, b1) = component();
    *a1.borrow_mut() = 10;
    *b1.borrow_mut() = 20;

    let (a2, b2) = component();
    assert_eq!(*a2.borrow(), 10);
    assert_eq!(*b2.borrow(), 20);
}
