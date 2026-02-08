use std::rc::Rc;

use hooks_rs::{mount_fiber, render_fiber, use_state};
use iced::{Element, widget::column};

use crate::{
    controls::{Controls, Filter},
    header::Header,
    input::NewTaskInput,
    tasks::{Task, TaskList},
};

mod controls;
mod header;
mod input;
mod tasks;

#[derive(Clone, Debug)]
enum Message {
    Refresh,
}

fn main() -> iced::Result {
    iced::application(Todos::new, Todos::update, Todos::view)
        .title("Todos - React Hooks")
        .window_size((500.0, 800.0))
        .run()
}

struct Todos;

impl Todos {
    fn new() -> Self {
        Self
    }
    fn update(&mut self, _: Message) {}
    fn view<'a>(&'a self) -> Element<'a, Message> {
        render_fiber(0, App, ()).unwrap()
    }
}

#[allow(non_snake_case)]
fn App(_: ()) -> Element<'static, Message> {
    let (tasks, set_tasks) = use_state(Vec::<Task>::new);
    let (input, set_input) = use_state(String::new);
    let (filter, set_filter) = use_state(|| Filter::All);

    let set_filter = Rc::new(set_filter);
    let set_tasks = Rc::new(set_tasks);
    let set_input = Rc::new(set_input);

    column![
        Header(),
        NewTaskInput((input.clone(), set_input, set_tasks.clone())),
        Controls((tasks.clone(), filter, set_filter)),
        TaskList((tasks, set_tasks, filter)),
    ]
    .spacing(20)
    .padding(40)
    .into()
}
