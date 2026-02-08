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
