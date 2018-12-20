mod util;

use util::{get_lines, get_file_path};

fn main() {
    let lines = get_lines(&get_file_path().unwrap());

    let part1 = part_1(&lines);
    //let part2 = part_2(&lines);

    println!("Part 1: {}", part1);
   // println!("Part 2: {}", part2);
}

fn part_1(lines: &Vec<String>) -> u32 {
    panic!("Not impemented");
}

fn part_2(lines: &Vec<String>) -> () {
    panic!("Not implemented");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> Vec<String> {
        vec![
            String::from("[1518-11-01 00:00] Guard #10 begins shift"),
            String::from("[1518-11-01 00:05] falls asleep"),
            String::from("[1518-11-01 00:25] wakes up"),
            String::from("[1518-11-01 00:30] falls asleep"),
            String::from("[1518-11-01 00:55] wakes up"),
            String::from("[1518-11-01 23:58] Guard #99 begins shift"),
            String::from("[1518-11-02 00:40] falls asleep"),
            String::from("[1518-11-02 00:50] wakes up"),
            String::from("[1518-11-03 00:05] Guard #10 begins shift"),
            String::from("[1518-11-03 00:24] falls asleep"),
            String::from("[1518-11-03 00:29] wakes up"),
            String::from("[1518-11-04 00:02] Guard #99 begins shift"),
            String::from("[1518-11-04 00:36] falls asleep"),
            String::from("[1518-11-04 00:46] wakes up"),
            String::from("[1518-11-05 00:03] Guard #99 begins shift"),
            String::from("[1518-11-05 00:45] falls asleep"),
            String::from("[1518-11-05 00:55] wakes up"),
        ]
    }

    #[test]
    fn part_1_assert_guard_10_sleeps_most_on_min_24() {
        let result = part_1(&get_test_data());
        assert_eq!(240, result)
    }
}
