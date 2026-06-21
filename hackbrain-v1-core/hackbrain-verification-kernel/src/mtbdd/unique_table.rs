use fxhash::FxHashMap;
use crate::mtbdd::manager::NodeId;

#[derive(Hash, Eq, PartialEq, Clone)]
struct Key {
    var: usize,
    low: NodeId,
    high: NodeId,
}

pub struct UniqueTable {
    table: FxHashMap<Key, NodeId>,
}

impl UniqueTable {
    pub fn new() -> Self {
        Self {
            table: FxHashMap::default(),
        }
    }

    pub fn lookup(&self, var: usize, low: &NodeId, high: &NodeId) -> Option<NodeId> {
        self.table.get(&Key {
            var,
            low: low.clone(),
            high: high.clone(),
        }).cloned()
    }

    pub fn insert(&mut self, var: usize, low: NodeId, high: NodeId, id: NodeId) {
        self.table.insert(Key { var, low, high }, id);
    }
}
