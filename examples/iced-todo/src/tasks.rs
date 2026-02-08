use std::{
    rc::Rc,
    sync::atomic::{AtomicU32, Ordering},
};

use hooks_rs::{render_fiber, unmount_fiber, use_state};
use iced::{
    Element,
    widget::{button, checkbox, keyed_column, row, text_input},
};

use crate::{Message, controls::Filter};

pub static TASK_ID: AtomicU32 = AtomicU32::new(1);

#[derive(Clone)]
pub struct Task {
    pub id: u32,
    pub completed: bool,
    pub description: String,
}

impl Task {
    pub fn new(description: String) -> Self {
        Self {
            id: TASK_ID.fetch_add(1, Ordering::Relaxed),
            completed: false,
            description,
        }
    }
}

#[allow(non_snake_case)]
pub fn TaskList(
    props: (
        Vec<Task>,
        Rc<dyn Fn(&dyn Fn(&Vec<Task>) -> Vec<Task>)>,
        Filter,
    ),
) -> Element<'static, Message> {
    let (tasks, set_tasks, filter) = props;

    let visible = tasks.iter().filter(|t| filter.matches(t));

    keyed_column(visible.map(|task| {
        (
            task.id,
            render_fiber(task.id, TaskItem, (task.clone(), set_tasks.clone())).unwrap(),
        )
    }))
    .spacing(10)
    .into()
}

#[allow(non_snake_case)]
fn TaskItem(
    props: (Task, Rc<dyn Fn(&dyn Fn(&Vec<Task>) -> Vec<Task>)>),
) -> Element<'static, Message> {
    let (task, set_tasks) = props;
    let (editing, set_editing) = use_state(|| false);

    let description = task.description.clone();
    let (text, set_text) = use_state(move || description);

    if editing {
        row![
            text_input("Edit task", &text)
                .on_input(move |v| {
                    set_text(&|_| v.clone());
                    Message::Refresh
                })
                .on_submit_with({
                    let set_tasks = set_tasks.clone();
                    move || {
                        set_tasks(&|prev| {
                            prev.iter()
                                .map(|t| {
                                    if t.id == task.id {
                                        Task {
                                            description: text.clone(),
                                            ..t.clone()
                                        }
                                    } else {
                                        t.clone()
                                    }
                                })
                                .collect()
                        });
                        set_editing(&|_| false);
                        Message::Refresh
                    }
                }),
            button("Delete").on_press_with(move || {
                set_tasks(&|prev| {
                    unmount_fiber(task.id);
                    prev.iter().filter(|t| t.id != task.id).cloned().collect()
                });
                Message::Refresh
            })
        ]
        .into()
    } else {
        row![
            checkbox(task.completed)
                .label(task.description)
                .on_toggle(move |v| {
                    set_tasks(&|prev| {
                        prev.iter()
                            .map(|t| {
                                if t.id == task.id {
                                    Task {
                                        completed: v,
                                        ..t.clone()
                                    }
                                } else {
                                    t.clone()
                                }
                            })
                            .collect()
                    });
                    Message::Refresh
                }),
            button("Edit").on_press_with(move || {
                set_editing(&|_| true);
                Message::Refresh
            })
        ]
        .into()
    }
}
