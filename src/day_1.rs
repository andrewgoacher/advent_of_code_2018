mod util;

use std::collections::HashMap;
use util::{get_lines, get_file_path};

pub fn part_1(path: &String) -> i32 {
    get_lines(path)
        .into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .sum()
}

pub fn part_2(path: &String) -> i32 {
    let lines = get_lines(path);
    internal_solve_proper(&lines)
}

fn internal_solve_proper(lines: &Vec<String>) -> i32 {
    let mut result: Option<i32> = None;
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut total = 0;
    map.insert(0, 1);
    while result == None {
        let (r, s) = internal_solve(total, &lines, &mut map);
        result = r;
        total = s;
    }
    result.unwrap()
}

fn internal_solve(
    total: i32,
    lines: &Vec<String>,
    map: &mut HashMap<i32, i32>,
) -> (Option<i32>, i32) {
    let mut sum = total.clone();
    for line in lines.into_iter() {
        let i = line.parse::<i32>().unwrap();
        sum += i;
        *map.entry(sum).or_insert(0) += 1;
        let v = map[&sum];
        if v == 2 {
            return (Some(sum), sum);
        }
    }

    let mut res = map.iter().filter(|&(_, v)| *v == 2);
    match res.nth(0) {
        None => (None, sum),
        Some(x) => {
            let (k, _) = x;
            (Some(k.clone()), sum)
        }
    }
}

fn main() {
    let file_path = get_file_path().unwrap();

    let part1 = part_1(&file_path);
    let part2 = part_2(&file_path);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_works_1() {
        let list = vec![String::from("+1"), String::from("-1")];

        let result = internal_solve_proper(&list);
        assert_eq!(0, result);
    }
    #[test]
    fn solve_works_2() {
        let list = vec![
            String::from("+3"),
            String::from("+3"),
            String::from("+4"),
            String::from("-2"),
            String::from("-4"),
        ];

        let result = internal_solve_proper(&list);
        assert_eq!(10, result);
    }
    #[test]
    fn solve_works_3() {
        let list = vec![
            String::from("-6"),
            String::from("+3"),
            String::from("+8"),
            String::from("+5"),
            String::from("-6"),
        ];

        let result = internal_solve_proper(&list);
        assert_eq!(5, result);
    }
    #[test]
    fn solve_works_4() {
        let list = vec![
            String::from("+7"),
            String::from("+7"),
            String::from("-2"),
            String::from("-7"),
            String::from("-4"),
        ];

        let result = internal_solve_proper(&list);
        assert_eq!(14, result);
    }
}
