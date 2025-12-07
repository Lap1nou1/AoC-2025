use regex::*;
use std::cmp::max;
use std::io::*;

pub(crate) const DAY: usize = 5;
pub fn run(input: &str) -> Result<()> {
    let (ranges_str, ingredients_str) = input
        .trim()
        .split_once("\n\n")
        .expect("Bad format for input file");

    let re = Regex::new(r"(\d+)-(\d+)").unwrap();

    let ranges = ranges_str.split("\n").map(|range| {
        let capt = re.captures(range).expect("Bad format for input file");
        (
            capt.get(1).unwrap().as_str().parse::<i64>().unwrap(),
            capt.get(2).unwrap().as_str().parse::<i64>().unwrap(),
        )
    });

    let ingredients = ingredients_str
        .split("\n")
        .map(|line| line.parse::<i64>().expect("Bad format for input file"));

    let ans1 = ingredients
        .filter(|ing| ranges.clone().any(|(min, max)| min <= *ing && *ing <= max))
        .count();

    let mut ranges_vec = ranges.collect::<Vec<_>>();

    ranges_vec.sort();

    let ans2 = ranges_vec.iter().scan(ranges_vec[0].0 - 1,
        |previous_end, (begin, end)| {
                let old = *previous_end;
                *previous_end = max(*end, *previous_end);

                if old < *begin {
                    Some(end - begin + 1)
                } else if old < *end {
                    Some(end - old)
                } else {
                    Some(0)
                }
        }).sum::<i64>();


    println!("First part answer: {ans1}");
    println!("Second part answer: {ans2}");

    Ok(())
}
