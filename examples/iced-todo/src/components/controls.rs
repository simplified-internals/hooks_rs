use hooks_rs::use_context;
use iced::{
    Alignment::{self},
    Length::Fill,
    widget::{button, text},
};

use crate::{
    Message,
    components::{FILTER_CTX, TASKS_CTX, tasks::Task},
    react::VNode,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Filter {
    #[default]
    All,
    Active,
    Completed,
}

impl Filter {
    pub fn matches(self, task: &Task) -> bool {
        match self {
            Filter::All => true,
            Filter::Active => !task.completed,
            Filter::Completed => task.completed,
        }
    }
}

#[allow(non_snake_case)]
pub fn Controls(_: ()) -> VNode<Message> {
    let (tasks, _) = use_context(*TASKS_CTX);

    let tasks_left = tasks.iter().filter(|t| !t.completed).count();

    VNode::row(
        vec![
            VNode::element(
                text!(
                    "{tasks_left} {} left",
                    if tasks_left == 1 { "task" } else { "tasks" }
                )
                .width(Fill),
            ),
            VNode::row(
                vec![
                    VNode::component("filter_button_all", filter_button, ("All", Filter::All)),
                    VNode::component(
                        "filter_button_active",
                        filter_button,
                        ("Active", Filter::Active),
                    ),
                    VNode::component(
                        "filter_button_completed",
                        filter_button,
                        ("Completed", Filter::Completed),
                    ),
                ],
                0,
                Alignment::Start,
            ),
        ],
        20,
        Alignment::Center,
    )
}

#[allow(non_snake_case)]
fn filter_button(props: (&'static str, Filter)) -> VNode<Message> {
    let (label, filter) = props;
    let (current_filter, set_current_filter) = use_context(*FILTER_CTX);

    let style = if filter == current_filter {
        iced::widget::button::primary
    } else {
        iced::widget::button::text
    };

    VNode::element(
        button(text(label))
            .style(style)
            .padding(8)
            .on_press_with(move || {
                set_current_filter(&|_| filter);
                Message::Refresh
            }),
    )
}
