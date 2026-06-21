use std::fs::File;
use std::io::Read;
use crate::core::wal::error::WalError;

pub struct RecoveryEngine;

impl RecoveryEngine {
    pub fn recover(mut file: File) -> Result<(u64, blake3::Hash), WalError> {
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;

        let mut cursor = 0;
        let mut last_valid_seq = 0;
        let mut last_hash = blake3::Hash::from([0u8; 32]);

        while cursor < buf.len() {
            if cursor + 64 > buf.len() {
                break; // partial tail → truncate
            }

            let chunk = &buf[cursor..];

            // pseudo decode (simplified)
            let seq = u64::from_le_bytes(chunk[0..8].try_into().unwrap());
            let payload_end = Self::find_commit_marker(chunk);

            if payload_end.is_none() {
                break; // no commit → truncate
            }

            let end_idx = payload_end.unwrap();
            
            // The payload itself starts after 8 bytes (seq) + 32 bytes (prev_hash) = 40 bytes
            // But we hash the whole byte array from 0 to end_idx.
            let hash = blake3::hash(&chunk[..end_idx]);

            // verify integrity
            if hash != last_hash && seq != 0 {
                break; // chain broken → truncate
            }

            last_valid_seq = seq;
            last_hash = hash;

            cursor += end_idx + 18; // skip marker (length of CAFE_F00D_DEADBEEF is 18 bytes)
        }

        // enforce prefix truncation physically
        file.set_len(cursor as u64)?;

        Ok((last_valid_seq, last_hash))
    }

    fn find_commit_marker(chunk: &[u8]) -> Option<usize> {
        // find b"CAFE_F00D_DEADBEEF"
        let marker = b"CAFE_F00D_DEADBEEF";
        for i in 0..chunk.len() {
            if i + marker.len() <= chunk.len() && &chunk[i..i+marker.len()] == marker {
                return Some(i);
            }
        }
        None
    }
}
