use std::{collections::HashMap, hash::Hash};

pub struct Arena<ID: ArenaID<Value>, Value> {
    pub values: HashMap<ID, Value>,
    next_id: ID,
}

pub struct ArenaContext<'a, ID: ArenaID<Value>, Value> {
    pub arena: &'a Arena<ID, Value>,
    pub id: ID,
}

impl<'a, ID: ArenaID<Value>, Value> ArenaContext<'a, ID, Value> {
    pub fn value(self) -> Option<&'a Value> {
        self.arena.get(self.id)
    }
}

pub trait ArenaID<Value>: Eq + Hash + Copy + Clone {
    fn generate_next(&self) -> Self;

    fn inside(self, arena: &Arena<Self, Value>) -> Option<&Value> {
        arena.get(self)
    }

    fn context<'a>(self, arena: &'a Arena<Self, Value>) -> ArenaContext<'a, Self, Value> {
        ArenaContext { arena, id: self }
    }

    fn inside_mut(self, tree: &mut Arena<Self, Value>) -> Option<&mut Value> {
        tree.get_mut(self)
    }
}

impl<ID: ArenaID<Value>, Value> Arena<ID, Value> {
    pub fn generate_id(&mut self) -> ID {
        loop {
            if let Some(_) = self.get(self.next_id) {
                self.next_id.generate_next();
            } else {
                return self.next_id;
            }
        }
    }

    pub fn get(&self, value: ID) -> Option<&Value> {
        self.values.get(&value)
    }

    pub fn get_mut(&mut self, value: ID) -> Option<&mut Value> {
        self.values.get_mut(&value)
    }

    pub fn add_value(&mut self, value: Value) -> Option<ID> {
        let id = self.generate_id();
        self.values.insert(id, value)?;

        Some(id)
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct IDu64(u64);

impl<Value> ArenaID<Value> for IDu64 {
    fn generate_next(&self) -> Self {
        IDu64(self.0 + 1)
    }
}
