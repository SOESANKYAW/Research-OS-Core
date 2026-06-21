use std::sync::Arc;
use crate::mtbdd::unique_table::UniqueTable;
use crate::mtbdd::cache::ApplyCache;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);

#[derive(Clone, Debug)]
pub struct MTBDDNode {
    pub var: usize,
    pub low: NodeId,
    pub high: NodeId,
    pub value: Option<f64>, // terminal if Some
}

pub struct MTBDDManager {
    pub unique: UniqueTable,
    pub cache: ApplyCache,
    pub next_id: usize,
    pub nodes: Vec<MTBDDNode>,
}

impl MTBDDManager {
    pub fn new() -> Self {
        Self {
            unique: UniqueTable::new(),
            cache: ApplyCache::new(),
            next_id: 0,
            nodes: Vec::new(),
        }
    }

    pub fn mk_terminal(&mut self, v: f64) -> NodeId {
        let id = NodeId(self.next_id);
        self.next_id += 1;

        self.nodes.push(MTBDDNode {
            var: usize::MAX,
            low: id.clone(),
            high: id.clone(),
            value: Some(v),
        });

        id
    }

    pub fn mk_node(&mut self, var: usize, low: NodeId, high: NodeId) -> NodeId {
        if low == high {
            return low;
        }

        if let Some(existing) = self.unique.lookup(var, &low, &high) {
            return existing;
        }

        let id = NodeId(self.next_id);
        self.next_id += 1;

        self.nodes.push(MTBDDNode {
            var,
            low: low.clone(),
            high: high.clone(),
            value: None,
        });

        self.unique.insert(var, low.clone(), high.clone(), id.clone());
        id
    }

    pub fn get(&self, id: &NodeId) -> &MTBDDNode {
        &self.nodes[id.0]
    }
}
