use std::fs::OpenOptions;
use std::io::{Write, Seek, SeekFrom};
use std::os::unix::fs::OpenOptionsExt;

pub struct WalWriter {
    file: std::fs::File,
    last_seq: u64,
}

impl WalWriter {
    pub fn open(path: &str) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .custom_flags(libc::O_SYNC) // IMPORTANT: reduce buffering ambiguity
            .open(path)?;

        Ok(Self {
            file,
            last_seq: 0,
        })
    }

    pub fn append(&mut self, payload: Vec<u8>) -> std::io::Result<()> {
        let seq_bytes = self.last_seq.to_le_bytes();

        // 1. write payload block
        self.file.write_all(&seq_bytes)?;
        self.file.write_all(&payload)?;

        // 2. FORCE data durability
        self.file.sync_data()?;

        // 3. commit marker
        self.file.write_all(b"COMMIT")?;

        // 4. FINAL barrier
        self.file.sync_all()?;

        self.last_seq += 1;

        Ok(())
    }
}
