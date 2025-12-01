mod day01;
use day01::{run, DAY};

use std::fs;

fn main() {
    let filename = format!("inputs/day{:0>2}_test_input", DAY);
    // let filename = format!("inputs/day{:0>2}_input", DAY);
    let input =
        fs::read_to_string(&filename).expect("You forgot to download the input didn't you?");

    let _ = run(&input);
}
