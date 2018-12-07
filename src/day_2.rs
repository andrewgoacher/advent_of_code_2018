use util::get_lines;
use std::collections::HashMap;

pub fn solve() -> u32 {
    let lines = get_lines("./data/day_2.txt");
    let hashmap = HashMap::new();
    let checksums = lines   
                    .into_iter()
                    .fold(hashmap,
                    |mut m, l| {
                        l.chars()
                            .fold(m,
                                |mut m2, c| {
                                    *m2.entry(c).or_insert(0) += 1; m2
                                });
                                m
                    });
    
    let twos_count = checksums.iter().filter(|&(k,v)| *v == 2).count() as u32;
    let threes_count = checksums.iter().filter(|&(k,v)| *v == 3).count() as u32;

    twos_count * threes_count
}