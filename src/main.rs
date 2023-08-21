use crate::parser::TopLevel;
use std::fs;

pub mod parser;

fn main() {
    let db_str = fs::read_to_string("./data/build-43953.json").expect("Unable to read DB");

    let top_level: TopLevel = serde_json::from_str(&db_str).unwrap();
    let serialized = serde_json::to_string(&top_level).unwrap();

    println!("{}", serialized);
}
