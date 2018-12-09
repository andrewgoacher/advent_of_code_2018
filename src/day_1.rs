use std::collections::HashMap;
use util::get_lines;

pub fn solve() -> i32 {
    let lines = get_lines("./data/day_1.txt");
    lines.into_iter().map(|x| x.parse::<i32>().unwrap()).sum()
}

pub fn solve_part2() -> i32 {
    let lines = get_lines("./data/day_1.txt");
    internal_solve_proper(&lines)
}

fn internal_solve_proper(lines: &Vec<String>) -> i32 {
    let mut result: Option<i32> = None;
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut total = 0;
    map.insert(0, 1);
    while result == None {
        let (r,s) = internal_solve(total, &lines, &mut map);
        result = r;
        total = s;
    }
    result.unwrap()
}

fn internal_solve( total: i32, lines: &Vec<String>, map: &mut HashMap<i32, i32>) -> (Option<i32>, i32) {
    let mut sum = total.clone();
    for line in lines.into_iter() {
        let i = line.parse::<i32>().unwrap();
        sum+= i;
        *map.entry(sum).or_insert(0) += 1;
        let v= map[&sum];
        if v == 2 {
            return (Some(sum), sum);
        }
    }

    let mut res = map.iter().filter(|&(k, v)| *v == 2);
    match res.nth(0) {
        None => (None, sum),
        Some(x) => {
            let (k, v) = x;
            (Some(k.clone()), sum)
        }
    }
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
