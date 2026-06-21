use crate::mtbdd::manager::MTBDDManager;

pub struct ReorderEngine;

impl ReorderEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn sifting(&mut self, _mgr: &mut MTBDDManager) {
        // NOTE:
        // Full dynamic variable reordering requires:
        // - swap adjacent variable levels
        // - local recompaction of unique table
        // - cost evaluation (node count heuristic)
        //
        // This is intentionally left as a hook for Phase B scaling.
    }
}
