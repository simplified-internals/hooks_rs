use std::rc::Rc;

use iced::{Element, widget::text_input};

use crate::{Message, tasks::Task};

#[allow(non_snake_case)]
pub fn NewTaskInput(
    props: (
        String,
        Rc<dyn Fn(&dyn Fn(&String) -> String)>,
        Rc<dyn Fn(&dyn Fn(&Vec<Task>) -> Vec<Task>)>,
    ),
) -> Element<'static, Message> {
    let (value, set_value, set_tasks) = props;

    text_input("What needs to be done?", &value)
        .on_input({
            let set_value = set_value.clone();
            move |v| {
                set_value(&|_| v.clone());
                Message::Refresh
            }
        })
        .on_submit_with(move || {
            if !value.is_empty() {
                set_tasks(&|prev| {
                    let mut next = prev.clone();
                    next.push(Task::new(value.clone()));
                    next
                });
                set_value(&|_| String::new());
            }
            Message::Refresh
        })
        .padding(15)
        .size(30)
        .into()
}
