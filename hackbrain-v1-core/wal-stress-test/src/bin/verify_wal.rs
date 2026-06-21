use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = match File::open("./wal.log") {
        Ok(f) => f,
        Err(_) => {
            println!("No wal.log found.");
            return;
        }
    };
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();

    let marker = b"COMMIT";
    let mut valid_count = 0;
    
    // Naive scan for COMMIT markers
    let mut i = 0;
    while i < data.len() {
        if i + marker.len() <= data.len() && &data[i..i+marker.len()] == marker {
            valid_count += 1;
            i += marker.len();
        } else {
            i += 1;
        }
    }

    println!("Recovered valid events (COMMIT markers found): {}", valid_count);
    println!("Total WAL size in bytes: {}", data.len());
}
