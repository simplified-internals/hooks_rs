mod component;
pub use component::*;

use iced::{Alignment, Element};

/// A intermediate representation of the UI. It is used so mounting/unmounting fibers is possible.
pub enum VNode<Message: 'static> {
    Element(Element<'static, Message>),
    Column {
        children: Vec<VNode<Message>>,
        spacing: u16,
        padding: u16,
    },
    Row {
        children: Vec<VNode<Message>>,
        spacing: u16,
        align_y: Alignment,
    },
    KeyedColumn {
        items: Vec<(u32, VNode<Message>)>,
        spacing: u16,
    },
    Component(ComponentNode<Message>),
}

impl<Message: 'static> VNode<Message> {
    pub fn element(el: impl Into<Element<'static, Message>>) -> Self {
        Self::Element(el.into())
    }

    pub fn column(children: Vec<Self>, spacing: u16, padding: u16) -> Self {
        Self::Column {
            children,
            spacing,
            padding,
        }
    }

    pub fn row(children: Vec<Self>, spacing: u16, align_y: Alignment) -> Self {
        Self::Row {
            children,
            spacing,
            align_y,
        }
    }

    pub fn keyed_column(items: Vec<(u32, Self)>, spacing: u16) -> Self {
        Self::KeyedColumn { items, spacing }
    }

    pub fn component<P>(
        key: impl Into<String>,
        fun: fn(P) -> VNode<Message>,
        props: P,
    ) -> VNode<Message>
    where
        P: 'static,
        Message: 'static,
    {
        VNode::Component(ComponentNode {
            key: key.into(),
            inner: Box::new(TypedComponent { fun, props }),
        })
    }
}
