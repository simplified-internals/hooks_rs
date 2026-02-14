use hooks_rs::use_context;
use iced::widget::text_input;

use crate::{
    Message,
    components::{INPUT_CTX, TASKS_CTX, tasks::Task},
    react::VNode,
};

#[allow(non_snake_case)]
pub fn NewTaskInput(_: ()) -> VNode<Message> {
    let (input, set_input) = use_context(*INPUT_CTX);
    let (_, set_tasks) = use_context(*TASKS_CTX);

    VNode::element(
        text_input("What needs to be done?", &input)
            .on_input(move |v| {
                set_input(|_| v.clone());
                Message::Refresh
            })
            .on_submit_with(move || {
                if !input.is_empty() {
                    set_tasks(|prev| {
                        let mut next = prev.clone();
                        next.push(Task::new(input.clone()));
                        next
                    });
                    set_input(|_| String::new());
                }
                Message::Refresh
            })
            .padding(15)
            .size(30),
    )
}
