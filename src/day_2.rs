use std::collections::HashMap;
use util::get_lines;

pub fn solve() -> u32 {
    let lines = get_lines("./data/day_2.txt");
    solve_internal(lines)
}

fn solve_internal(lines: Vec<String>) -> u32 {
    let mut twos = 0;
    let mut threes = 0;

    for line in lines.into_iter() {
        let hashmap = line.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
        let tc = hashmap.iter().filter(|&(k, v)| *v == 2).count() as u32;
        let thc = hashmap.iter().filter(|&(k, v)| *v == 3).count() as u32;
        if  tc > 0 {
            twos += 1;
        }

        if thc > 0 {
            threes += 1;
        }
    }

    twos * threes
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
}
