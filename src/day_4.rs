mod util;

use util::{get_lines, get_file_path};

fn main() {
    let lines = get_lines(&get_file_path().unwrap());

    let part1 = part_1(&lines);
    let part2 = part_2(&lines);

   // println!("Part 1: {}", part1);
   // println!("Part 2: {}", part2);
}

fn part_1(lines: &Vec<String>) -> () {
    panic!("Not impemented");
}

fn part_2(lines: &Vec<String>) -> () {
    panic!("Not implemented");
}

#[cfg(test)]
mod tests {
    use super::*;
}
