use iced::{Alignment::Center, Element, Length::Fill, widget::text};

use crate::Message;

#[allow(non_snake_case)]
pub fn Header() -> Element<'static, Message> {
    text("todos").size(100).width(Fill).align_x(Center).into()
}
