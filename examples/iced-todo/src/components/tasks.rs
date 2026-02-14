use std::sync::atomic::{AtomicU32, Ordering};

use hooks_rs::{use_context, use_state};
use iced::widget::{button, checkbox, row, text_input};

use crate::{
    Message,
    components::{FILTER_CTX, TASKS_CTX},
    react::VNode,
};

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
pub fn TaskList(_: ()) -> VNode<Message> {
    let (tasks, _) = use_context(*TASKS_CTX);
    let (filter, _) = use_context(*FILTER_CTX);

    let mut items = Vec::new();

    for task in tasks.into_iter().filter(|t| filter.matches(t)) {
        let key = task.id.to_string();
        items.push((task.id, VNode::component(key, TaskItem, task)));
    }

    VNode::keyed_column(items, 10)
}

#[allow(non_snake_case)]
fn TaskItem(task: Task) -> VNode<Message> {
    let (_, set_tasks) = use_context(*TASKS_CTX);

    let (editing, set_editing) = use_state(|| false);

    let description = task.description.clone();
    let (text, set_text) = use_state(move || description);

    if editing {
        VNode::element(row![
            text_input("Edit task", &text)
                .on_input(move |v| {
                    set_text(|_| v.clone());
                    Message::Refresh
                })
                .on_submit_with(move || {
                    set_tasks(|prev| {
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
                    set_editing(|_| false);
                    Message::Refresh
                }),
            button("Delete").on_press_with(move || {
                set_tasks(|prev| prev.iter().filter(|t| t.id != task.id).cloned().collect());
                Message::Refresh
            })
        ])
    } else {
        let task_description = task.description.clone();
        VNode::element(row![
            checkbox(task.completed)
                .label(task_description)
                .on_toggle(move |v| {
                    set_tasks(|prev| {
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
                set_editing(|_| true);
                Message::Refresh
            })
        ])
    }
}
