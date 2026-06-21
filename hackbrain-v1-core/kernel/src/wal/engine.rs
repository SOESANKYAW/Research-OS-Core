use crate::wal::types::WALRecord;
use crate::wal::hash::{canonical_json, hash};
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Debug, thiserror::Error)]
pub enum WalError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub struct WALLEngine {
    pub last_hash: String,
    pub sequence: u64,
    pub filepath: String,
}

impl WALLEngine {
    pub fn new(filepath: String) -> Self {
        WALLEngine {
            last_hash: "GENESIS".to_string(),
            sequence: 0,
            filepath,
        }
    }

    /// Appends a new record to the WAL, enforcing the hash chain.
    ///
    /// Proof of Append Correctness:
    /// 1. `sequence` is strictly monotonic.
    /// 2. `prev_hash` is cryptographically chained to `last_hash`.
    /// 3. `event_hash` is computed deterministically using a canonicalized JSON payload.
    /// 4. The record is persisted to disk immediately to prevent silent loss.
    pub fn append(&mut self, mut record: WALRecord) -> Result<String, WalError> {
        // Enforce monotonic sequence
        record.sequence = self.sequence + 1;

        // Enforce chain integrity
        record.prev_hash = self.last_hash.clone();

        let canonical = canonical_json(&record.event);
        let computed_hash = hash(&format!("{}{}", record.prev_hash, canonical));

        record.event_hash = computed_hash.clone();

        self.last_hash = computed_hash.clone();
        self.sequence += 1;

        self.write_to_disk(&record)?;

        Ok(computed_hash)
    }

    fn write_to_disk(&self, record: &WALRecord) -> Result<(), WalError> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.filepath)?;

        let line = serde_json::to_string(record)?;
        writeln!(file, "{}", line)?;

        // Ensure OS flush for true durability
        file.sync_data()?;

        Ok(())
    }
}
