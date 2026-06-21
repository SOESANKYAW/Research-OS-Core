use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct ReplayIndex {
    /// Last fully committed WAL sequence number
    pub last_seq: u64,

    /// Hash of the last successfully verified event
    pub last_event_hash: [u8; 32],

    /// Hash pointer into WAL chain (for fast resumption validation)
    pub wal_tip_hash: [u8; 32],
}

#[derive(Debug)]
pub enum WalError {
    IoError(String),
    Corruption,
}

#[derive(Debug)]
pub enum IndexError {
    IoError(String),
    NotFound,
}

#[derive(Debug)]
pub enum RecoveryError {
    WalError(WalError),
    IndexError(IndexError),
    ChainMismatch,
}

impl From<WalError> for RecoveryError {
    fn from(err: WalError) -> Self {
        RecoveryError::WalError(err)
    }
}

impl From<IndexError> for RecoveryError {
    fn from(err: IndexError) -> Self {
        RecoveryError::IndexError(err)
    }
}

/// The key abstraction boundary for the causal cursor over the WAL.
pub trait WalCursor {
    type Event;

    /// Seek to first event AFTER a given sequence
    fn seek_after(&mut self, seq: u64) -> Result<(), WalError>;

    /// Fetch next event in causal order
    fn next(&mut self) -> Option<Result<Self::Event, WalError>>;

    /// Validate hash continuity against previous event
    fn verify_chain(
        &self,
        prev_hash: &[u8; 32],
        event: &Self::Event,
    ) -> Result<[u8; 32], WalError>;

    /// Returns current cursor position (not state!)
    fn position(&self) -> u64;
}

/// Crash-Safe Persistence Layer
pub trait ReplayIndexStore {
    fn load(&self) -> Result<ReplayIndex, IndexError>;

    fn persist(&self, index: &ReplayIndex) -> Result<(), IndexError>;

    /// atomic_update MUST guarantee either old index or new index is visible
    /// — never partial write
    fn atomic_update(&self, index: &ReplayIndex) -> Result<(), IndexError>;
}

/// The exact boot sequence contract.
pub struct WalRecovery<C: WalCursor> {
    pub cursor: C,
    pub index_store: Box<dyn ReplayIndexStore>,
}

impl<C: WalCursor> WalRecovery<C> {
    pub fn recover(&mut self) -> Result<ReplayIndex, RecoveryError> {
        let index = self.index_store.load()?;

        self.cursor.seek_after(index.last_seq)?;

        // Validate chain continuity before resuming
        if let Some(event) = self.cursor.next() {
            let event = event?;

            let expected_hash = self.cursor.verify_chain(&index.last_event_hash, &event)?;

            // Strict continuity gate
            if expected_hash != index.wal_tip_hash {
                return Err(RecoveryError::ChainMismatch);
            }
        }

        Ok(index)
    }
}
