use std::env;
use std::fs::File;
use std::io::prelude::*;

use serde::{Deserialize, Serialize};

use sheltie::index::Index;
use sheltie::searcher::Searcher;

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
    let args: Vec<String> = env::args().collect();
    let index_path = match args.len() {
        1 => panic!("Usage: cargo run --example do_query examples/data/segment.doc < examples/data/queries.txt"),
        2 => &args[1],
        _ => panic!("Usage: cargo run --example do_query examples/data/segment.doc < examples/data/queries.txt"),
    };

    // Load out the index.
    let mut file = File::open(index_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let index: Index = bincode::deserialize(&buffer[..]).unwrap();

    // Search by queries from stdin.
    let searcher = Searcher::new(&index);
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let query = parse_json(&line).map_err(|err| format!("Failed to parse JSON: {}", err))?;
        println!("{:?}", query.query);

        // Only support TOP_10.
        let res = searcher.search(&query.query, 10);
        println!("{:?}", res.len());
    }

    Ok(())
}
