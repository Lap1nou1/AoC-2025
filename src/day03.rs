use num::pow;
use std::cmp::max;
use std::io::*;

fn find_batteries(bank: &str, remaning: usize) -> u64 {
    if remaning == 0 {
        0
    } else {
        bank[..bank.len() - remaning + 1]
            .as_bytes()
            .iter()
            .map(|v| (v - ('0' as u8)) as u64)
            .enumerate()
            .reduce(|(acc_i, acc_v), (i, v)| if acc_v < v { (i, v) } else { (acc_i, acc_v) })
            .map(|(i, v)| v * pow(10, remaning - 1) + find_batteries(&bank[i + 1..], remaning - 1))
            .expect("Empry string")
    }
}

pub(crate) const DAY: usize = 3;
pub fn run(input: &str) -> Result<()> {
    let ans1 = input
        .trim()
        .split("\n")
        .map(|bank| find_batteries(bank, 2))
        .sum::<u64>();

    let ans2 = input
        .trim()
        .split("\n")
        .map(|bank| find_batteries(bank, 12))
        .sum::<u64>();

    println!("First part answer: {ans1}");
    println!("Second part answer: {ans2}");

    Ok(())
}
