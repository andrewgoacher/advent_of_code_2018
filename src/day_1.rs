use util::get_lines;

pub fn solve() -> i32 {
    let lines = get_lines("./data/day_1.txt");
    lines.into_iter()
                .map(|x| x.parse::<i32>().unwrap())
                .sum()
}