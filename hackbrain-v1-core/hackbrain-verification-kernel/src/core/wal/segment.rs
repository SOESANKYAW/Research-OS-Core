use std::fs::File;
use std::io::{Write, Seek, SeekFrom};

pub struct WalSegment {
    pub file: File,
    pub start_seq: u64,
    pub end_seq: u64,
}

impl WalSegment {
    pub fn append(&mut self, data: &[u8]) -> std::io::Result<u64> {
        let offset = self.file.seek(SeekFrom::End(0))?;

        self.file.write_all(data)?;
        self.file.sync_data()?; // Phase A: data durability

        Ok(offset)
    }

    pub fn commit_marker(&mut self) -> std::io::Result<()> {
        self.file.write_all(b"CAFE_F00D_DEADBEEF")?;
        self.file.sync_all()?; // Phase B: commit visibility barrier

        Ok(())
    }
}
