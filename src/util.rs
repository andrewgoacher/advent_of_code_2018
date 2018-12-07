use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;

pub fn get_lines(file: &str) -> Vec<String> {
    let f = File::open(file).unwrap();
    let mut buffered_file_reader = BufReader::new(&f);
    let mut lines  = Vec::new();

    for line in buffered_file_reader.lines() {
        let l = line.unwrap();
        lines.push(l);
    }

    lines
}