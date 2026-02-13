use iced::{Alignment::Center, Length::Fill, widget::text};

use crate::{Message, react::VNode};

#[allow(non_snake_case)]
pub fn Header(_: ()) -> VNode<Message> {
    VNode::element(text("todos").size(100).width(Fill).align_x(Center))
}
