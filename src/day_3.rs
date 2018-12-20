mod util;

use std::collections::{HashMap, HashSet};
use util::{get_file_path, get_lines};
extern crate regex;
use regex::Regex;
use std::fmt;

struct GridPoint {
    x: u32,
    y: u32,
    claim_id: u32,
}

impl GridPoint {
    fn new(x: u32, y: u32, id: u32) -> GridPoint {
        GridPoint {
            x: x,
            y: y,
            claim_id: id,
        }
    }
}

fn get_capture(re: &Regex, line: &String) -> (u32, u32, u32, u32, u32) {
    let caps = re.captures(&line).unwrap();
    let x = caps
        .get(2)
        .map_or(0, |m| m.as_str().parse::<u32>().unwrap());
    let y = caps
        .get(3)
        .map_or(0, |m| m.as_str().parse::<u32>().unwrap());
    let w = caps
        .get(4)
        .map_or(0, |m| m.as_str().parse::<u32>().unwrap());
    let h = caps
        .get(5)
        .map_or(0, |m| m.as_str().parse::<u32>().unwrap());
    let claim_id = caps
        .get(1)
        .map_or(0, |m| m.as_str().parse::<u32>().unwrap());

    (x, y, w, h, claim_id)
}

fn get_cells(lines: &Vec<String>) -> Vec<GridPoint> {
    // #1 @ 179,662: 16x27
    let re = Regex::new(r"#(\d+)+ @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    lines.into_iter().fold(Vec::new(), |mut acc, l| {
        let (x, y, w, h, id) = get_capture(&re, &l);

        for i in x..(x + w) {
            for j in y..(y + h) {
                acc.push(GridPoint::new(i, j, id));
            }
        }
        acc
    })
}

fn part_1(lines: &Vec<String>) -> u32 {
    let count: u32 = get_cells(&lines)
        .into_iter()
        .fold(HashMap::new(), |mut acc, cell| {
            *acc.entry((cell.x, cell.y)).or_insert(0) += 1;
            acc
        }).into_iter()
        .filter(|(_, v)| *v > 1)
        .count() as u32;

    count
}

struct CellItem {
    count: u32,
    claims: HashSet<u32>,
}

impl CellItem {
    fn new() -> CellItem {
        CellItem {
            count: 0,
            claims: HashSet::new()
        }
    }
}

impl fmt::Debug for CellItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CellItem: {{count: {}, claims: {:?}}}",
            self.count, self.claims
        )
    }
}

fn part_2(lines: &Vec<String>) -> u32 {
    let map = get_cells(&lines).into_iter().fold(
        HashMap::<(u32, u32), CellItem>::new(),
        |mut acc, cell| {
            {
                let item = acc.entry((cell.x, cell.y)).or_insert(CellItem::new());
                if item.claims.insert(cell.claim_id) {
                    item.count += 1;
                }
            }
            acc
        },
    );

    let unwanted_claims: HashSet<u32> =
        map.iter()
            .filter(|(_, item)| item.count > 1)
            .fold(HashSet::new(), |mut set, (_, item)| {
                for claim in item.claims.iter() {
                    set.insert(claim.clone());
                }
                set
            });

    let all_claim = map
        .iter()
        .fold(HashSet::new(), |mut acc, (_, item)| {
            for claim in item.claims.iter() {
                acc.insert(claim);
            }
            acc
        })
        .into_iter()
        .filter(|claim| unwanted_claims.contains(claim) == false);

        let claim = all_claim.into_iter()
        .nth(0)
        .unwrap();

    claim.clone()
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

    #[test]
    fn part_2_solve_with_inputs_have_correct_unconflicting_claim() {
        let inputs = vec![
            String::from("#1 @ 1,3: 4x4"),
            String::from("#2 @ 3,1: 4x4"),
            String::from("#3 @ 5,5: 2x2"),
        ];

        let result = part_2(&inputs);
        assert_eq!(3, result)
    }
}
