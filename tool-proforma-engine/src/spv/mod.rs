pub mod ambassadors_d1;
pub mod ambassadors_d2;
pub mod bencal;

/// Merge a `_derivation` block into the top-level JSON object for audit purposes.
pub fn audited_json<T: serde::Serialize>(data: &T, derivation: serde_json::Value) -> String {
    let mut map = serde_json::Map::new();
    map.insert("_derivation".to_string(), derivation);
    if let serde_json::Value::Object(data_map) = serde_json::to_value(data).unwrap() {
        for (k, v) in data_map {
            map.insert(k, v);
        }
    }
    serde_json::to_string_pretty(&serde_json::Value::Object(map)).unwrap()
}
