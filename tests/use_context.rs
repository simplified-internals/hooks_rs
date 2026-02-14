use std::sync::LazyLock;

use hooks_rs::{
    Context, FiberStoreError, call_fiber, create_context, mount_fiber, provide_context, use_context,
};

static CTX: LazyLock<Context<i32>> = LazyLock::new(|| create_context());

fn parent(_: ()) -> i32 {
    provide_context(*CTX, 5);
    call_fiber("root/child", ()).unwrap()
}

fn child(_: ()) -> i32 {
    use_context(*CTX)
}

#[test]
fn context_flows_down_call_stack() -> Result<(), FiberStoreError> {
    mount_fiber(None, "root", parent)?;
    mount_fiber(Some("root".into()), "root/child", child)?;

    let v = call_fiber::<(), i32>("root", ())?;
    assert_eq!(v, 5);
    Ok(())
}
