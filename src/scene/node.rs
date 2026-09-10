use std::any::Any;

use crate::util::arena::{Arena, ArenaContext, IDu64};

pub struct Node<T> {
    pub value: T,
    pub parent: Option<NodeID>,
    pub children: Vec<NodeID>,
}

pub type AnyNode = Node<Box<dyn Any>>;

pub type NodeID = IDu64;

pub type NodeTree = Arena<NodeID, AnyNode>;

pub type NodeContext<'a, ID, Value> = ArenaContext<'a, ID, Value>;

impl NodeTree {
    pub fn parent_id(&self, child: NodeID) -> Option<NodeID> {
        self.get(child)?.parent
    }

    pub fn child_ids(&self, parent: NodeID) -> Option<&Vec<NodeID>> {
        (&self.get(parent)).map(|parent_node| &parent_node.children)
    }

    pub fn add_child(&mut self, parent: NodeID, child: NodeID) -> Option<()> {
        self.get_mut(parent)?.children.push(child);

        Some(())
    }
}
