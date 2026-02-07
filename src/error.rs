pub enum FiberStoreError {
    FiberAlreadyExists(u32),
    FiberDoesntExist(u32),
    FiberTypeMismatch(u32),
}
