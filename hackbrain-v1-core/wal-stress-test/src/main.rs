use std::time::Duration;
use std::thread::sleep;

mod wal;
use wal::WalWriter;

fn main() {
    let mut wal = WalWriter::open("./wal.log").unwrap();

    let mut i: u64 = 0;

    loop {
        let payload = format!("event_payload_{}", i).into_bytes();

        match wal.append(payload) {
            Ok(_) => {
                println!("[OK] appended event {}", i);
            }
            Err(e) => {
                eprintln!("[WAL ERROR] {:?}", e);
                break;
            }
        }

        i += 1;

        // small deterministic delay (important for crash interleaving diversity)
        sleep(Duration::from_millis(10));
    }
}
