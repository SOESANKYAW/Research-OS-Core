pub mod codec;
pub mod segment;
pub mod writer;
pub mod recovery;
pub mod error;

pub type Seq = u64;

#[derive(Debug, Clone)]
pub struct WalEvent {
    pub seq: Seq,
    pub payload: Vec<u8>,
    pub prev_hash: blake3::Hash,
}
