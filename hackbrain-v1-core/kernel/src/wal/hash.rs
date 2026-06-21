use sha2::{Sha256, Digest};
use serde::Serialize;
use serde_json::Value;

/// Canonicalizes a JSON value by sorting keys.
/// This is a simplified version; in production, use a dedicated crate for JSON canonicalization.
pub fn canonical_json<T: Serialize>(value: &T) -> String {
    let mut val: Value = serde_json::to_value(value).unwrap();
    sort_keys(&mut val);
    serde_json::to_string(&val).unwrap()
}

fn sort_keys(value: &mut Value) {
    match value {
        Value::Array(arr) => {
            for v in arr.iter_mut() {
                sort_keys(v);
            }
        }
        Value::Object(map) => {
            let mut sorted = serde_json::Map::new();
            let mut keys: Vec<String> = map.keys().cloned().collect();
            keys.sort();
            for k in keys {
                let mut v = map.remove(&k).unwrap();
                sort_keys(&mut v);
                sorted.insert(k, v);
            }
            *map = sorted;
        }
        _ => {}
    }
}

pub fn hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}
