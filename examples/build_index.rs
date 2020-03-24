extern crate rustsearch;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::io::BufRead;
use Result;

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    id: String,
    text: String,
}

fn parse_json(data: &str) -> serde_json::Result<Record> {
    let record: Record = serde_json::from_str(data)?;
    Ok(record)
}

fn main() -> Result<(), String> {
    let stdin = std::io::stdin();
    let mut index = rustsearch::Index::new();
    for line in stdin.lock().lines() {
        let line = line.map_err(|err| format!("Failed to get line from stdin: {}", err))?;
        if line.trim().is_empty() {
            continue;
        }

        let record = parse_json(&line).map_err(|err| format!("Failed to parse JSON: {}", err))?;
        println!("{}: {}", record.id, record.text);
        (&mut index).add(record.text);
    }
    Ok(())
}
