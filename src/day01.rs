use std::io::*;

pub(crate) const DAY: usize = 1;
pub fn run(input: &str) -> Result<()> {
    // Parse the input
    let moves = input.lines().map(|line| {
        let val = line[1..].parse::<i32>().unwrap();
        if line.chars().next().unwrap() == 'R' {
            val
        } else {
            -val
        }
    });

    let mut ans1 = 0;
    let mut ans2 = 0;

    // Position can have value in (-100, 100)
    let mut position = 50;

    for change in moves {
        let old_pos = position;

        position += change;

        // We test if we have passed through 0 in three different cases
        if position <= -100 || position >= 100 {
            ans2 += (position / 100).abs()
        }
        // If we are right in the middle of our range
        if position == 0 {
            ans2 += 1
        }
        // If we've gone from positive to negative, and vice-versa
        if position * old_pos < 0 {
            ans2 += 1
        }

        // We put position in our range, by property of the modulos (not reminder), we stay on our
        // range
        position %= 100;

        if position == 0 {
            ans1 += 1;
        }
    }

    println!("First part answer: {ans1}");
    println!("Second part answer: {ans2}");

    Ok(())
}
