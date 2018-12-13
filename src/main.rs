mod util;
mod day_1;
mod day_2;

fn main() {
    let day_1_result = day_1::solve();
    println!("day 1 - part 1: {}",day_1_result);

    let day_1_part_2 = day_1::solve_part2();
    println!("day 1 - part 2: {}", day_1_part_2);

    let day_2_result = day_2::solve();
    println!("day 2 - part 1: {}", day_2_result);

    ()
}