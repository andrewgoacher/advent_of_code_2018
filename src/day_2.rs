mod util;

use std::collections::HashMap;
use util::{get_file_path, get_lines, hamming_distance};

pub fn part_1(path: &String) -> u32 {
    let lines = get_lines(path);
    solve_internal(lines)
}

fn part_2(path: &String) -> String {
    let lines = get_lines(path);

    solve_part_2_internal(&lines)
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
        if tc > 0 {
            twos += 1;
        }

        if thc > 0 {
            threes += 1;
        }
    }

    twos * threes
}

fn solve_part_2_internal(lines: &Vec<String>) -> String {
    let map = lines
        .into_iter()
        .fold(HashMap::new(), |mut acc, la| {
            for lb in lines.into_iter() {
                *acc.entry((la, lb, hamming_distance(&la, &lb))).or_insert(0) += 1;
            }
            acc
        });

    let ((ax,bx,_),_) = 
        map.iter()
        .filter(|&((_,_,hd),_)| *hd < 2 && *hd > 0)
        .nth(0).unwrap();

        let char_map : HashMap<char, u32> = 
            ax.chars().into_iter()
            .zip(bx.chars().into_iter())
            .fold(HashMap::new(), |mut acc, (ac,bc)| {
            *acc.entry(ac).or_insert(0) += 1;
            *acc.entry(bc).or_insert(0) += 1;
            acc
        });

        let unloved_chars : Vec<char> = char_map.into_iter().filter(|&(_,v)| v == 1).map(|(k,_)| k).collect();
        let s : String = ax.chars().into_iter().filter(|c| unloved_chars.contains(&c) == false).collect();
        s
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
            String::from("abcde"),
            String::from("fghij"),
            String::from("klmno"),
            String::from("pqrst"),
            String::from("fguij"),
            String::from("axcye"),
            String::from("wvxyz"),
        ];

        let result = solve_part_2_internal(&strings);
        assert_eq!(result, String::from("fgij"));
    }
}
