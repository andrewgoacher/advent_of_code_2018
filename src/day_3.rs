mod util;

use std::collections::HashMap;
use util::{get_file_path, get_lines};

fn part_1(lines: &Vec<String>) -> u32 {
    String::from("Unsolved")
}

fn part_2(lines: &Vec<String>) -> String {
    String::from("Unsolved")
}

fn main() {
    let lines = get_lines(get_file_path().unwrap());

    let part_1 = part_1(&lines);
    println!("Part 1: {}", part_1);

    let part_2 = part_2(&lines);
    println!("Part 2: {}", part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_solve_with_inputs_have_conflicting_claims() {
        let inputs = vec![[
            String::from("#1 @ 1,3: 4x4"),
            String::from("#2 @ 3,1: 4x4"),
            String::from("#3 @ 5,5: 2x2"),
        ]];

        let result = part_1(&inputs);
        assert_eq!(4, result)
    }
}
