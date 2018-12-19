mod util;

use std::collections::HashMap;
use util::{get_file_path, get_lines};
extern crate regex;
use regex::Regex;

struct GridCell {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

impl GridCell {
    fn new(x:u32, y:u32, w:u32, h:u32) -> GridCell {
        GridCell {
            x: x,
            y: y,
            w: w,
            h: h
        }
    }
}

fn get_cells(lines: &Vec<String>) -> Vec<GridCell> {
    // #1 @ 179,662: 16x27
    let re = Regex::new(r"#\d+ @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    lines
        .into_iter()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            let x = caps
                .get(1)
                .map_or(0, |m| m.as_str().parse::<u32>().unwrap());
            let y = caps
                .get(2)
                .map_or(0, |m| m.as_str().parse::<u32>().unwrap());
            let w = caps
                .get(3)
                .map_or(0, |m| m.as_str().parse::<u32>().unwrap());
            let h = caps
                .get(4)
                .map_or(0, |m| m.as_str().parse::<u32>().unwrap());

            GridCell::new(x,y,w,h)
        }).collect()
}

fn part_1(lines: &Vec<String>) -> u32 {
    let count: u32= get_cells(&lines)
    .into_iter()
        .fold(Vec::new(), |mut col, cell| {
            for i in cell.x..(cell.x + cell.w) {
                for j in cell.y..(cell.y + cell.h) {
                    col.push((i, j))
                }
            }
            col
        }).into_iter()
        .fold(HashMap::new(), |mut acc, (x, y)| {
            *acc.entry((x, y)).or_insert(0) += 1;
            acc
        }).into_iter()
        .filter(|(_, v)| *v > 1)
        .count() as u32;

        count
}

fn part_2(lines: &Vec<String>) -> String {
    String::from("Unsolved")
}

fn main() {
    let lines = get_lines(&get_file_path().unwrap());

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
        let inputs = vec![
            String::from("#1 @ 1,3: 4x4"),
            String::from("#2 @ 3,1: 4x4"),
            String::from("#3 @ 5,5: 2x2"),
        ];

        let result = part_1(&inputs);
        assert_eq!(4, result)
    }
}
