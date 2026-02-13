use std::sync::LazyLock;

use hooks_rs::{Context, SetStateAction, create_context, provide_context, use_state};

use crate::{
    Message,
    components::{Controls, Filter, Header, NewTaskInput, Task, TaskList},
    react::VNode,
};

pub static TASKS_CTX: LazyLock<Context<(Vec<Task>, SetStateAction<Vec<Task>>)>> =
    LazyLock::new(|| create_context());
pub static INPUT_CTX: LazyLock<Context<(String, SetStateAction<String>)>> =
    LazyLock::new(|| create_context());
pub static FILTER_CTX: LazyLock<Context<(Filter, SetStateAction<Filter>)>> =
    LazyLock::new(|| create_context());

#[allow(non_snake_case)]
pub fn App(_: ()) -> VNode<Message> {
    let (tasks, set_tasks) = use_state(Vec::<Task>::new);
    let (input, set_input) = use_state(String::new);
    let (filter, set_filter) = use_state(|| Filter::All);

    provide_context(*TASKS_CTX, (tasks, set_tasks));
    provide_context(*INPUT_CTX, (input, set_input));
    provide_context(*FILTER_CTX, (filter, set_filter));

    VNode::column(
        vec![
            VNode::component("header", Header, ()),
            VNode::component("new_task_input", NewTaskInput, ()),
            VNode::component("controls", Controls, ()),
            VNode::component("task_list", TaskList, ()),
        ],
        20,
        40,
    )
}
