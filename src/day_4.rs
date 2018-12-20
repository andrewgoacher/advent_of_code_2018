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
            String::from("Date   ID   Minute"),
            String::from("000000000011111111112222222222333333333344444444445555555555"),
            String::from("012345678901234567890123456789012345678901234567890123456789"),
            String::from("11-01  #10  .....####################.....#########################....."),
            String::from("11-02  #99  ........................................##########.........."),
            String::from("11-03  #10  ........................#####..............................."),
            String::from("11-04  #99  ....................................##########.............."),
            String::from("11-05  #99  .............................................##########.....")
        ]
    }

    #[test]
    fn part_1_assert_guard_10_sleeps_most_on_min_24() {
        let result = part_1(&get_test_data());
        assert_eq!(240, result)
    }
}
