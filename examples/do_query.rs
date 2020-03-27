use rustsearch;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

use rustsearch::index::Index;
use rustsearch::index::Searcher;

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    query: String,
    tags: Vec<String>,
}

fn parse_json(data: &str) -> serde_json::Result<Record> {
    let record: Record = serde_json::from_str(data)?;
    Ok(record)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load out the index.
    let mut file = File::open("segment.doc")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let index: Index = bincode::deserialize(&buffer[..]).unwrap();
    let searcher = Searcher::new(&index);

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let query = parse_json(&line).map_err(|err| format!("Failed to parse JSON: {}", err))?;
        println!("{:?}", query.query);

        // Only support TOP_10
        let res = searcher.search(&query.query, 10);
        println!("{:?}", res.len());
    }

    Ok(())
}
