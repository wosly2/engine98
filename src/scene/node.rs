use std::{any::Any, collections::HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeID(u64);

pub struct Node<T: Any> {
    pub value: Box<T>,
    pub parent: Option<NodeID>,
    pub children: Vec<NodeID>,
}

type AnyNode = Node<Box<dyn Any>>;

pub struct NodeTree {
    pub nodes: HashMap<NodeID, AnyNode>,
    next_id: u64,
}

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
    pub fn generate_id(&mut self) -> NodeID {
        loop {
            if let Some(_) = self.get(NodeID(self.next_id)) {
                self.next_id += 1;
            } else {
                return NodeID(self.next_id);
            }
        }
    }

    pub fn get(&self, node: NodeID) -> Option<&AnyNode> {
        self.nodes.get(&node)
    }

    pub fn get_mut(&mut self, node: NodeID) -> Option<&mut AnyNode> {
        self.nodes.get_mut(&node)
    }

    pub fn parent_id(&self, child: NodeID) -> Option<NodeID> {
        self.get(child)?.parent
    }

    pub fn child_ids(&self, parent: NodeID) -> Option<&Vec<NodeID>> {
        (&self.get(parent)).map(|parent_node| &parent_node.children)
    }

    pub fn add_node(&mut self, node: AnyNode) -> Option<NodeID> {
        let id = self.generate_id();
        self.nodes.insert(id, node)?;

        Some(id)
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
