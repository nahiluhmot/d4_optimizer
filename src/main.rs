use crate::parser::TopLevel;
use std::env::args;
use std::fs;

pub mod parser;

fn main() {
    let mut argv = args();

    let _progname = argv.next();

    let filename: String = argv.next().expect("No file name given");
    let db_str = fs::read_to_string(filename).expect("Unable to read DB");

    let top_level: TopLevel = serde_json::from_str(&db_str).unwrap();
    let serialized = serde_json::to_string_pretty(&top_level).unwrap();

    println!("{}", serialized);
}
