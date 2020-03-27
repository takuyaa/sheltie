use bincode;
use rustsearch;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::{BufRead, Write};

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
    let stdin = std::io::stdin();
    let mut index = rustsearch::Index::new();
    for line in stdin.lock().lines() {
        let line = line.map_err(|err| format!("Failed to get line from stdin: {}", err))?;
        if line.trim().is_empty() {
            continue;
        }
        let record = parse_json(&line).map_err(|err| format!("Failed to parse JSON: {}", err))?;
        index.add(&record.text);
    }
    // Write out the index.
    let encoded: Vec<u8> = bincode::serialize(&index).unwrap();
    let mut file = File::create("segment.doc")?;
    file.write_all(&encoded)?;
    file.flush()?;

    Ok(())
}
