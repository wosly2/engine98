use std::{collections::HashMap, hash::Hash};

pub struct Arena<ID: ArenaID, Value> {
    pub values: HashMap<ID, Value>,
    next_id: ID,
}

pub struct ArenaContext<'a, ID: ArenaID, Value> {
    pub arena: &'a Arena<ID, Value>,
    pub id: ID,
}

impl<'a, ID: ArenaID, Value> ArenaContext<'a, ID, Value> {
    pub fn value(self) -> Option<&'a Value> {
        self.arena.get(self.id)
    }
}

pub trait ArenaID: Eq + Hash + Copy + Clone {
    fn generate_next(&self) -> Self;

    fn inside<Value>(self, arena: &Arena<Self, Value>) -> Option<&Value> {
        arena.get(self)
    }

    fn context<'a, Value>(self, arena: &'a Arena<Self, Value>) -> ArenaContext<'a, Self, Value> {
        ArenaContext { arena, id: self }
    }

    fn inside_mut<Value>(self, arena: &mut Arena<Self, Value>) -> Option<&mut Value> {
        arena.get_mut(self)
    }
}

impl<ID: ArenaID, Value> Arena<ID, Value> {
    // simplisitic for now
    pub fn generate_id(&mut self) -> ID {
        while let Some(_) = self.get(self.next_id) {
            self.next_id = self.next_id.generate_next();
        }
        self.next_id
    }

    pub fn get(&self, value: ID) -> Option<&Value> {
        self.values.get(&value)
    }

    pub fn get_mut(&mut self, value: ID) -> Option<&mut Value> {
        self.values.get_mut(&value)
    }

    pub fn push(&mut self, value: Value) -> ID {
        let id = self.generate_id();
        self.values.insert(id, value);
        id
    }

    pub fn pop(&mut self, id: ID) -> Option<Value> {
        self.values.remove(&id)
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct IDu64(u64);

impl ArenaID for IDu64 {
    fn generate_next(&self) -> Self {
        // very very simplistic
        IDu64(self.0 + 1)
    }
}
