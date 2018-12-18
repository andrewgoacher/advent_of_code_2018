mod util;

use std::collections::HashMap;
use util::{get_lines,get_file_path};

pub fn part_1(path: &String) -> u32 {
    let lines = get_lines(path);
    solve_internal(lines)
}

fn part_2(path: &String) -> String {
    let lines = get_lines(path);

    solve_part_2_internal(lines)
}

fn solve_internal(lines: Vec<String>) -> u32 {
    let mut twos = 0;
    let mut threes = 0;

    for line in lines.into_iter() {
        let hashmap = line.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
        let tc = hashmap.iter().filter(|&(_, v)| *v == 2).count() as u32;
        let thc = hashmap.iter().filter(|&(_, v)| *v == 3).count() as u32;
        if  tc > 0 {
            twos += 1;
        }

        if thc > 0 {
            threes += 1;
        }
    }

    twos * threes
}

fn solve_part_2_internal(lines: Vec<String>) -> String {
    let ordered_lines : Vec<String> = lines.into_iter().map(|l| {
       let mut chars : Vec<char> = l.chars().collect();
       chars.sort_by(|a,b| a.cmp(b)) ;
       let len = chars.len()-1;
       let c : String = chars
            .into_iter()
            .enumerate()
            .filter(|&(i,_)| i < len )
            .map(|(_,ch)| ch)
            .collect();
       c
    }).collect();

    // todo: learn rust properly
    println!("{:?}", ordered_lines);

    let res : Vec<String> = ordered_lines.into_iter().fold(HashMap::new(), |mut acc, line| {
        *acc.entry(line).or_insert(1) += 1;
        acc
    })
    .iter()
    .filter(|&(_,v)| *v > 1)
    .map(|(k,_)| k.clone())
    .collect();

    res[0].clone()
}

fn main() {
    let file_path = get_file_path().unwrap();

    let part_1 = part_1(&file_path);
    println!("Part 1: {}", part_1);

    let part_2 = part_2(&file_path);
    println!("Part 2: {}", part_2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_works() {
        let strings = vec![
            String::from("abcdef"),
            String::from("bababc"),
            String::from("abbcde"),
            String::from("abcccd"),
            String::from("aabcdd"),
            String::from("abcdee"),
            String::from("ababab"),
        ];
        let result = solve_internal(strings);

        assert_eq!(result, 12);
    }

    #[test]
    fn part_2_solve() {
        let strings = vec![
            String::from("abcdef"),
            String::from("abcdeg"),
            String::from("avbsde")
        ];

        let result = solve_part_2_internal(strings);
        assert_eq!(result, String::from("abcde"));
    }
}
