use util::get_lines;
use std::collections::HashMap;

struct CheckSum {
    two: i32,
    three: i32
}

fn get_checksum(input: &string) -> CheckSum {
    let hm = input.chars()
        .fold(HashMap::new(),
         |mut m, c| { *m.entry(c).or_insert(0) += 1; m});
    let mut two = 0;
    let mut three = 0;

    for (k,v) in hm {
        if v == 2 {
            two +=1;
        } else if v == 3 {
            three += 1;
        }
    }
    CheckSum {two = two, three = three}
}

pub fn solve() -> i32 {
    let lines = get_lines("./data/day_2.txt");
    let hashmap = HashMap::new();
    let checksums = lines   
                    .into_iter()

    0
}