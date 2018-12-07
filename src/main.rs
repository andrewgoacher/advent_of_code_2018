mod util;
mod day_1;
mod day_2;

fn main() {
    let day_1_result = day_1::solve();
    println!("{}",day_1_result);

    let day_2_result = day_2::solve();
    println!("{}", day_2_result);

    ()
}