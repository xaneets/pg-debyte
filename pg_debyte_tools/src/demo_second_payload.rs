use bincode::Options;
use hex::encode;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct DemoRecord {
    id: u32,
    text: String,
    flag: bool,
}

fn main() {
    let record = DemoRecord {
        id: 1,
        text: "second".to_string(),
        flag: true,
    };

    let bytes = bincode::DefaultOptions::new()
        .with_limit(32 * 1024 * 1024)
        .serialize(&record)
        .expect("serialize demo payload");

    println!("{}", encode(bytes));
}
