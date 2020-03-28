use std::fs::File;
use std::io::{Read, BufReader};

use xplan::parser::parse;
use xplan::dot::render;

fn main() {
    let file_path = "input.yml";
    let file = File::open(file_path).unwrap();

    let mut buf_reader = BufReader::new(file);
    let mut yaml = String::new();
    buf_reader.read_to_string(&mut yaml).unwrap();

    let store = parse(&yaml).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    let mut stdout = std::io::stdout();
    render(&mut stdout, &store).unwrap();
}

