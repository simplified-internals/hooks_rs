use std::fmt::Display;

#[derive(Debug)]
pub enum FiberStoreError {
    FiberAlreadyExists(String),
    FiberDoesntExist(String),
    FiberTypeMismatch(String),
    ParentDoesNotExist(String),
}

impl Display for FiberStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FiberStoreError::FiberAlreadyExists(id) => {
                write!(f, "Fiber with id {} already exists", id)
            }
            FiberStoreError::FiberDoesntExist(id) => {
                write!(f, "Fiber with id {} doesn't exist", id)
            }
            FiberStoreError::FiberTypeMismatch(id) => {
                write!(f, "Fiber with id {} has a different type", id)
            }
            FiberStoreError::ParentDoesNotExist(id) => {
                write!(f, "Parent fiber with id {} doesn't exist", id)
            }
        }
    }
}
