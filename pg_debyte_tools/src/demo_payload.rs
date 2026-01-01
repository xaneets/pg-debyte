use bincode::Options;
use hex::encode;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct DemoRecord {
    id: u32,
    label: String,
}

fn main() {
    let record = DemoRecord {
        id: 1,
        label: "demo".to_string(),
    };

    let bytes = bincode::DefaultOptions::new()
        .with_limit(32 * 1024 * 1024)
        .serialize(&record)
        .expect("serialize demo payload");

    println!("{}", encode(bytes));
}
