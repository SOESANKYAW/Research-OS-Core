use crc32fast::Hasher;
use crate::core::wal::WalEvent;

pub struct EncodedEvent {
    pub bytes: Vec<u8>,
    pub crc32: u32,
    pub hash: blake3::Hash,
}

pub fn encode(event: &WalEvent) -> EncodedEvent {
    let mut bytes = Vec::new();

    bytes.extend_from_slice(&event.seq.to_le_bytes());
    bytes.extend_from_slice(event.prev_hash.as_bytes());
    bytes.extend_from_slice(&event.payload);

    let crc32 = {
        let mut h = Hasher::new();
        h.update(&bytes);
        h.finalize()
    };

    let hash = blake3::hash(&bytes);

    EncodedEvent { bytes, crc32, hash }
}
