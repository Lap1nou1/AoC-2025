use num::pow;
use regex::Regex;
use std::collections::HashSet;
use std::{cmp, io, iter};

fn search_invalid_ids(start: &str, end: &str, n: usize) -> impl Iterator<Item = i64> {
    // n is the number of times the invalided ids are repeated
    // Since we search for translated information, we start only with the first
    let start_i64 = if start.len() % n == 0 {
        let first = &start[..start.len() / n];
        let first_i64 = first.parse::<i64>().unwrap();

        // We test whether the repetition of n "first" can be in the range
        if first.repeat(n).parse::<i64>().unwrap() > start.parse::<i64>().unwrap() {
            first_i64
        } else {
            first_i64 + 1
        }
    } else {
        // If we can't subdivise by n, then the smallest valid value has to be 10^...
        pow(10i64, start.len() / n)
    };

    let end_i64 = end.parse::<i64>().unwrap();

    (start_i64..).map_while(move |i| {
        let code = i.to_string().repeat(n).parse::<i64>().unwrap();

        if code <= end_i64 {
            Some(code)
        } else {
            None
        }
    })
}

pub(crate) const DAY: usize = 2;
pub fn run(input: &str) -> Result<(), ()> {
    let re = Regex::new(r"(\d+)-(\d+)").unwrap();
    let ranges = input.split(',').map(|line| {
        let capt = re.captures(line).unwrap();
        (capt.get(1).unwrap().as_str(), capt.get(2).unwrap().as_str())
    });

    let ans1 = ranges
        .clone()
        .map(|(a, b)| search_invalid_ids(a, b, 2))
        .flatten()
        .sum::<i64>();

    let mut invalid_ids = HashSet::<i64>::new();

    for (start, end) in ranges {
        for n in 2..end.len() + 1 {
            for id in search_invalid_ids(start, end, n) {
                invalid_ids.insert(id);
            }
        }
    }

    let ans2 = invalid_ids.iter().sum::<i64>();

    println!("First part answer: {ans1}");
    println!("Second part answer: {ans2}");

    Ok(())
}
