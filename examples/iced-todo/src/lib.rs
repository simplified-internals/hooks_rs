pub mod components;
pub mod react;

//

use std::cell::RefCell;

use crate::{components::App, react::create_root};
use iced::Element;

#[derive(Clone, Debug)]
pub enum Message {
    Refresh,
}

pub struct Todos {
    root: RefCell<react::Root<(), Message>>,
}

impl Todos {
    pub fn new() -> Self {
        Self {
            root: RefCell::new(create_root("root", App)),
        }
    }
    pub fn update(&mut self, _: Message) {}
    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        self.root.borrow_mut().view(())
    }
}
