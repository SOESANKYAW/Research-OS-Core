use std::sync::{Arc, Mutex};
use crate::core::wal::{WalEvent, segment::WalSegment, error::WalError};

pub struct WalWriter {
    pub segment: Arc<Mutex<WalSegment>>,
    pub last_hash: blake3::Hash,
    pub seq: u64,
}

impl WalWriter {
    pub fn append(&mut self, payload: Vec<u8>) -> Result<(), WalError> {
        let event = WalEvent {
            seq: self.seq,
            payload,
            prev_hash: self.last_hash,
        };

        let encoded = crate::core::wal::codec::encode(&event);

        let mut seg = self.segment.lock().unwrap();

        // 1. write payload
        seg.file.write_all(&encoded.bytes)?;

        // 2. fsync data barrier
        seg.file.sync_data()?;

        // 3. write commit marker
        seg.file.write_all(b"CAFE_F00D_DEADBEEF")?;

        // 4. final fsync barrier
        seg.file.sync_all()?;

        // 5. update in-memory causal state ONLY after durability
        self.last_hash = encoded.hash;
        self.seq += 1;

        Ok(())
    }
}
