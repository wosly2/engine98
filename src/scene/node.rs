use std::any::Any;

use crate::util::arena::{Arena, IDu64};

pub struct Node<T> {
    pub value: Box<T>,
    pub parent: Option<NodeID>,
    pub children: Vec<NodeID>,
}

type AnyNode = Node<Box<dyn Any>>;

type NodeID = IDu64;

type NodeTree = Arena<NodeID, AnyNode>;

pub struct NodeContext<'a> {
    tree: &'a NodeTree,
    id: NodeID,
}

impl NodeID {
    pub fn inside(self, tree: &NodeTree) -> Option<&AnyNode> {
        tree.get(self)
    }

    pub fn context<'a>(self, tree: &'a NodeTree) -> NodeContext<'a> {
        NodeContext { tree, id: self }
    }

    pub fn inside_mut(self, tree: &mut NodeTree) -> Option<&mut AnyNode> {
        tree.get_mut(self)
    }
}

impl NodeTree {
    pub fn parent_id(&self, child: NodeID) -> Option<NodeID> {
        self.get(child)?.parent
    }

    pub fn child_ids(&self, parent: NodeID) -> Option<&Vec<NodeID>> {
        (&self.get(parent)).map(|parent_node| &parent_node.children)
    }

    pub fn add_child(&mut self, parent: NodeID, child: NodeID) -> Option<()> {
        parent.inside_mut(self)?.children.push(child);

        Some(())
    }
}

impl<'a> NodeContext<'a> {
    pub fn node(self) -> Option<&'a AnyNode> {
        self.tree.get(self.id)
    }
}
