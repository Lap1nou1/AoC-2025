use std::collections::HashSet;
use std::io::*;

pub(crate) const DAY: usize = 4;
pub fn run(input: &str) -> Result<()> {
    let rolls = input
        .split("\n")
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '@')
                .map(move |(x, _)| (x as i64, y as i64))
        })
        .flatten();

    let mut rolls_position = HashSet::<(i64, i64)>::from_iter(rolls);

    let neighbors = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let mut ans1 = 0;
    let mut ans2 = 0;

    let mut added = 1;
    while added > 0 {
        let accessible_rolls = rolls_position
            .iter()
            .filter(|(px, py)| {
                neighbors
                    .iter()
                    .filter(|(vx, vy)| rolls_position.contains(&(px + vx, py + vy)))
                    .count()
                    < 4
            })
            .map(|(x, y)| (*x, *y))
            .collect::<Vec<_>>();

        for (x, y) in accessible_rolls.iter() {
            rolls_position.remove(&(*x, *y));
        }

        added = accessible_rolls.len();

        if ans1 == 0 {
            ans1 = added
        }
        ans2 += added;
    }

    println!("First part answer {ans1}");
    println!("Second part answer {ans2}");

    Ok(())
}
