use hooks_rs::SetStateAction;
use iced::{
    Alignment::Center,
    Element,
    Length::Fill,
    widget::{button, row, text},
};

use crate::{Message, tasks::Task};

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
pub fn Controls(props: (Vec<Task>, Filter, SetStateAction<Filter>)) -> Element<'static, Message> {
    let (tasks, current_filter, set_filter) = props;

    let tasks_left = tasks.iter().filter(|t| !t.completed).count();

    row![
        text!(
            "{tasks_left} {} left",
            if tasks_left == 1 { "task" } else { "tasks" }
        )
        .width(Fill),
        row![
            filter_button(("All", current_filter, Filter::All, set_filter)),
            filter_button(("Active", current_filter, Filter::Active, set_filter)),
            filter_button(("Completed", current_filter, Filter::Completed, set_filter)),
        ]
        .spacing(10)
    ]
    .spacing(20)
    .align_y(Center)
    .into()
}

#[allow(non_snake_case)]
fn filter_button(
    props: (&'static str, Filter, Filter, SetStateAction<Filter>),
) -> Element<'static, Message> {
    let (label, current_filter, filter, set_filter) = props;

    let style = if filter == current_filter {
        iced::widget::button::primary
    } else {
        iced::widget::button::text
    };

    button(text(label))
        .style(style)
        .padding(8)
        .on_press_with(move || {
            set_filter(&|_| filter);
            Message::Refresh
        })
        .into()
}
