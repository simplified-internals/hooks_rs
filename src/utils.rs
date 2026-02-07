use std::any::Any;

pub trait DynEq: Any {
    fn eq_dyn(&self, other: &dyn Any) -> bool;
}

impl<T: PartialEq + 'static> DynEq for T {
    fn eq_dyn(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<T>().map_or(false, |o| self == o)
    }
}

pub(crate) fn deps_changed(old: &[Box<dyn DynEq>], new: &[Box<dyn DynEq>]) -> bool {
    if old.len() != new.len() {
        return true;
    }
    for (o, n) in old.iter().zip(new.iter()) {
        if !o.eq_dyn(&**n) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod deps_changed {
    use super::*;

    #[test]
    fn same_primitives_are_equal() {
        let old: Vec<Box<dyn DynEq>> = vec![Box::new(42), Box::new(100u64)];
        let new: Vec<Box<dyn DynEq>> = vec![Box::new(42), Box::new(100u64)];

        assert!(!deps_changed(&old, &new));
    }

    #[test]
    fn different_primitives_are_detected() {
        let old: Vec<Box<dyn DynEq>> = vec![Box::new(42), Box::new(100u64)];
        let new: Vec<Box<dyn DynEq>> = vec![Box::new(43), Box::new(100u64)];

        assert!(deps_changed(&old, &new));
    }

    #[test]
    fn custom_structs_work() {
        #[derive(PartialEq, Debug)]
        struct MyStruct {
            x: i32,
            y: String,
        }

        let a = MyStruct {
            x: 1,
            y: "hi".into(),
        };
        let b = MyStruct {
            x: 1,
            y: "hi".into(),
        };
        let c = MyStruct {
            x: 2,
            y: "hi".into(),
        };

        let old: Vec<Box<dyn DynEq>> = vec![Box::new(a)];
        let same: Vec<Box<dyn DynEq>> = vec![Box::new(b)];
        let changed: Vec<Box<dyn DynEq>> = vec![Box::new(c)];

        assert!(!deps_changed(&old, &same));
        assert!(deps_changed(&old, &changed));
    }

    #[test]
    fn different_lengths_detected() {
        let old: Vec<Box<dyn DynEq>> = vec![Box::new(1)];
        let new: Vec<Box<dyn DynEq>> = vec![Box::new(1), Box::new(2)];

        assert!(deps_changed(&old, &new));
    }
}
