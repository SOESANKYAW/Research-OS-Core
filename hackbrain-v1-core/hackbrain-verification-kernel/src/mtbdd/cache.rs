use fxhash::FxHashMap;
use crate::mtbdd::manager::NodeId;

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct ApplyKey {
    pub op: u8,
    pub a: NodeId,
    pub b: NodeId,
}

pub struct ApplyCache {
    pub table: FxHashMap<ApplyKey, NodeId>,
}

impl ApplyCache {
    pub fn new() -> Self {
        Self {
            table: FxHashMap::default(),
        }
    }

    pub fn get(&self, key: &ApplyKey) -> Option<NodeId> {
        self.table.get(key).cloned()
    }

    pub fn insert(&mut self, key: ApplyKey, value: NodeId) {
        self.table.insert(key, value);
    }
}
