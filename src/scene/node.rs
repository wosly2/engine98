use std::{any::Any, collections::HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeID(u64);

pub struct Node {
    pub value: Box<dyn Any>,
    pub parent: Option<NodeID>,
    pub children: Vec<NodeID>,
}

pub struct NodeTree {
    pub nodes: HashMap<NodeID, Node>,
    next_id: u64,
}

impl NodeID {
    pub fn inside(self, tree: &NodeTree) -> Option<&Node> {
        tree.get(self)
    }

    pub fn inside_mut(self, tree: &mut NodeTree) -> Option<&mut Node> {
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

    pub fn get(&self, node: NodeID) -> Option<&Node> {
        self.nodes.get(&node)
    }

    pub fn get_mut(&mut self, node: NodeID) -> Option<&mut Node> {
        self.nodes.get_mut(&node)
    }

    pub fn parent_id(&self, child: NodeID) -> Option<NodeID> {
        self.get(child)?.parent
    }

    pub fn child_ids(&self, parent: NodeID) -> Option<&Vec<NodeID>> {
        (&self.get(parent)).map(|parent_node| &parent_node.children)
    }

    pub fn add_node(&mut self, node: Node) -> Option<NodeID> {
        let id = self.generate_id();
        self.nodes.insert(id, node)?;

        Some(id)
    }

    pub fn add_child(&mut self, parent: NodeID, child: NodeID) -> Option<()> {
        parent.inside_mut(self)?.children.push(child);

        Some(())
    }
}
