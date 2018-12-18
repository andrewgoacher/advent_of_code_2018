use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

extern crate clap;
use self::clap::{Arg,App};

pub fn get_file_path() -> Option<String> {
    let matches = App::new("Advent of Code 2018")
                    .version("1.0")
                    .about("At some point, I will finish this challenge")
                    .arg(Arg::with_name("data")
                        .short("d")
                        .long("data")
                        .value_name("DATA")
                        .takes_value(true)).get_matches();
    
    match matches.value_of("data") {
        None => None,
        Some(x) => Some(String::from(x))
    }
}

pub fn get_lines(file: &str) -> Vec<String> {
    let f = File::open(file).unwrap();
    let buffered_file_reader = BufReader::new(&f);
    let mut lines  = Vec::new();

    for line in buffered_file_reader.lines() {
        let l = line.unwrap();
        lines.push(l);
    }

    lines
}

pub fn hamming_distance(a: &String, b: &String) -> usize {
    if a.len() != b.len() {
        panic!("strings must be same length");
    }

    a
    .chars()
    .into_iter()
    .enumerate()
    .zip(b.chars().into_iter().enumerate())
    .map(|(ac, bc)| (ac,bc))
    .filter(|(ac,bc)| *ac != *bc)
    .count()
}