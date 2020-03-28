use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::io::BufRead;
use std::path::Path;

use rustsearch::index::IndexWriter;

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    id: String,
    text: String,
}

fn parse_json(data: &str) -> serde_json::Result<Record> {
    let record: Record = serde_json::from_str(data)?;
    Ok(record)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let index_dir_path = match args.len() {
        1 => panic!(
            "Usage: cargo run --example build_index examples/data < examples/data/corpus.json"
        ),
        2 => &args[1],
        _ => panic!(
            "Usage: cargo run --example build_index examples/data < examples/data/corpus.json"
        ),
    };

    // Open the IndexWriter.
    let mut writer = IndexWriter::new(Path::new(index_dir_path));

    // Read documents from stding and index it.
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.map_err(|err| format!("Failed to get line from stdin: {}", err))?;
        if line.trim().is_empty() {
            continue;
        }
        let record = parse_json(&line).map_err(|err| format!("Failed to parse JSON: {}", err))?;
        writer.add(&record.text);
    }

    // Write out the index.
    writer.export_index()?;

    Ok(())
}
