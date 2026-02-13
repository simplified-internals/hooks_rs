use std::collections::HashSet;

use hooks_rs::{call_fiber, get_children_ids, mount_fiber, unmount_fiber};
use iced::{Element, widget};

use crate::react::node::{ComponentNode, VNode};

pub fn reconcile(parent_id: &str, new_children: HashSet<String>) {
    let prev_children: HashSet<String> = get_children_ids(parent_id.to_string())
        .unwrap_or_default()
        .into_iter()
        .collect();

    for removed in prev_children.difference(&new_children) {
        unmount_fiber(removed.to_string());
    }
}

pub struct Root<P, Message: 'static> {
    pub(crate) root_id: String,
    pub(crate) root_fun: fn(P) -> VNode<Message>,
}

pub fn create_root<P, Message: 'static>(
    root_id: impl Into<String>,
    root_fun: fn(P) -> VNode<Message>,
) -> Root<P, Message>
where
    P: 'static,
{
    Root {
        root_id: root_id.into(),
        root_fun,
    }
}

impl<P, Message: 'static> Root<P, Message>
where
    P: 'static,
{
    pub fn view(&mut self, props: P) -> Element<'static, Message> {
        match mount_fiber(None, self.root_id.clone(), self.root_fun) {
            _ => {}
        }

        let vnode = call_fiber(self.root_id.clone(), props).expect("Failed to call root fiber");

        let root_children = collect_child_ids(&vnode, &self.root_id);
        reconcile(&self.root_id, root_children);

        let id = self.root_id.clone();
        self.render_vnode(&id, vnode)
    }

    pub(crate) fn render_vnode(
        &mut self,
        parent_id: &str,
        vnode: VNode<Message>,
    ) -> Element<'static, Message> {
        match vnode {
            VNode::Element(el) => el,
            VNode::Column {
                children,
                spacing,
                padding,
            } => {
                let rendered: Vec<Element<'static, Message>> = children
                    .into_iter()
                    .map(|c| self.render_vnode(parent_id, c))
                    .collect();

                let col = widget::column(rendered)
                    .spacing(spacing as f32)
                    .padding(padding as f32);
                col.into()
            }
            VNode::Row {
                children,
                spacing,
                align_y,
            } => {
                let rendered: Vec<Element<'static, Message>> = children
                    .into_iter()
                    .map(|c| self.render_vnode(parent_id, c))
                    .collect();

                let row = widget::row(rendered)
                    .spacing(spacing as f32)
                    .align_y(align_y);
                row.into()
            }
            VNode::KeyedColumn { items, spacing } => {
                let rendered: Vec<(u32, Element<'static, Message>)> = items
                    .into_iter()
                    .map(|(k, v)| (k, self.render_vnode(parent_id, v)))
                    .collect();

                let col = widget::keyed_column(rendered).spacing(spacing as f32);
                col.into()
            }
            VNode::Component(node) => self.render_component(parent_id, node),
        }
    }

    pub(crate) fn render_component(
        &mut self,
        parent_id: &str,
        node: ComponentNode<Message>,
    ) -> Element<'static, Message> {
        let child_id = format!("{parent_id}/{}", node.key);

        node.inner
            .mount(parent_id, &child_id)
            .unwrap_or_else(|e| panic!("Failed to mount fiber `{child_id}`: {e}"));

        let child_vnode = node
            .inner
            .call(child_id.clone())
            .unwrap_or_else(|e| panic!("Failed to call fiber `{child_id}`: {e}"));

        let child_children = collect_child_ids(&child_vnode, &child_id);
        reconcile(&child_id, child_children);

        self.render_vnode(&child_id, child_vnode)
    }
}

pub(crate) fn collect_child_ids<Message: 'static>(
    vnode: &VNode<Message>,
    parent_id: &str,
) -> HashSet<String> {
    let mut out = HashSet::new();
    collect_child_ids_impl(vnode, parent_id, &mut out);
    out
}

pub(crate) fn collect_child_ids_impl<Message: 'static>(
    vnode: &VNode<Message>,
    parent_id: &str,
    out: &mut HashSet<String>,
) {
    match vnode {
        VNode::Element(_) => {}
        VNode::Column { children, .. } => {
            for c in children {
                collect_child_ids_impl(c, parent_id, out);
            }
        }
        VNode::Row { children, .. } => {
            for c in children {
                collect_child_ids_impl(c, parent_id, out);
            }
        }
        VNode::KeyedColumn { items, .. } => {
            for (_k, v) in items {
                collect_child_ids_impl(v, parent_id, out);
            }
        }
        VNode::Component(node) => {
            out.insert(format!("{parent_id}/{}", node.key));
        }
    }
}
