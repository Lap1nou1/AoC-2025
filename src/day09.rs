use regex::Regex;
use std::cmp::{max, min};

// Note: This code doesn't work (probably an error with the angles). However, it gave me the
// correct answer and I wanted to sleep

fn get_manhattan_dist(x: &(i64, i64), y: &(i64, i64)) -> i64 {
    (x.0 - y.0).abs() + (x.1 - y.1).abs()
}

fn get_area(x: &(i64, i64), y: &(i64, i64)) -> i64 {
    ((x.0 - y.0).abs() + 1) * ((x.1 - y.1).abs() + 1)
}

enum Orientation {
    North,
    East,
    South,
    West,
}

fn is_clockwise(o1: &Orientation, o2: &Orientation) -> bool {
    match (o1, o2) {
        (Orientation::North, Orientation::East)
        | (Orientation::East, Orientation::South)
        | (Orientation::South, Orientation::West)
        | (Orientation::West, Orientation::North) => true,
        _ => false,
    }
}

fn get_orientation(before: &(i64, i64), next: &(i64, i64)) -> Orientation {
    if before.0 > next.0 {
        Orientation::West
    } else if before.0 < next.0 {
        Orientation::East
    } else if before.1 > next.1 {
        Orientation::North
    } else {
        Orientation::South
    }
}

fn dot_product(lhs: &(i64, i64), rhs: &(i64, i64)) -> i64 {
    lhs.0 * rhs.0 + lhs.1 * rhs.1
}

pub(crate) const DAY: usize = 9;
pub fn run(input: &str) -> Result<(), ()> {
    let re = Regex::new(r"(\d+),(\d+)").unwrap();
    let points = input
        .lines()
        .map(|line| {
            let cap = re.captures(line).expect("Bad format");
            (
                cap.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                cap.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let nb_points = points.len();

    let corner_top_left = points.iter().fold((i64::MAX, i64::MAX), |acc, pt| {
        (min(acc.0, pt.0), min(acc.1, pt.1))
    });
    let corner_bottom_left = points.iter().fold((i64::MAX, 0), |acc, pt| {
        (min(acc.0, pt.0), max(acc.1, pt.1))
    });
    let corner_top_right = points.iter().fold((0, i64::MAX), |acc, pt| {
        (max(acc.0, pt.0), min(acc.1, pt.1))
    });
    let corner_bottom_right = points
        .iter()
        .fold((0, 0), |acc, pt| (max(acc.0, pt.0), max(acc.1, pt.1)));

    let tl = points
        .iter()
        .min_by_key(|pt| get_manhattan_dist(&corner_top_left, pt))
        .expect("No points in the input");
    let (bl_ind, bl) = points
        .iter()
        .enumerate()
        .min_by_key(|(_, pt)| get_manhattan_dist(&corner_bottom_left, pt))
        .expect("No points in the input");
    let tr = points
        .iter()
        .min_by_key(|pt| get_manhattan_dist(&corner_top_right, pt))
        .expect("No points in the input");
    let br = points
        .iter()
        .min_by_key(|pt| get_manhattan_dist(&corner_bottom_right, pt))
        .expect("No points in the input");

    let ans1 = max(get_area(tl, br), get_area(tr, bl));

    // Part 2
    // First, we determine an orientation which will let us have an easy way to find which angles
    // are interior/exterior
    //
    // To do this, we search which direction (between +1 and -1) in the vector points give us a
    // higher point
    let mut dir = 1;

    if bl.1 >= points[(bl_ind + 1) % nb_points].1 {
        dir = -1
    }

    let prev_ind = |i: usize| {
        if dir == -1 {
            (i + 1) % nb_points
        } else {
            if i == 0 {
                nb_points - 1
            } else {
                i - 1
            }
        }
    };
    let next_ind = |i: usize| {
        if dir == 1 {
            (i + 1) % nb_points
        } else {
            if i == 0 {
                nb_points - 1
            } else {
                i - 1
            }
        }
    };

    // Now, we go through each turn to see if the 90° happens inside the polygon (ie if the
    // rotation is clockwise)
    let orientations =
        Vec::from_iter((0..nb_points).map(|i| get_orientation(&points[i], &points[next_ind(i)])));
    let is_interior = Vec::from_iter(
        (0..nb_points).map(|i| is_clockwise(&orientations[i], &orientations[next_ind(i)])),
    );

    let mut ans2 = 0;

    for i in 0..nb_points {
        for j in i + 1..nb_points {
            // We search if the angle from i "can see" j
            let v1 = (
                points[next_ind(i)].0 - points[i].0,
                points[next_ind(i)].1 - points[i].1,
            );
            let v2 = (
                points[prev_ind(i)].0 - points[i].0,
                points[prev_ind(i)].1 - points[i].1,
            );
            let v3 = (points[j].0 - points[i].0, points[j].1 - points[i].1);

            let in_acute_part = dot_product(&v1, &v3) >= 0 && dot_product(&v2, &v3) >= 0;

            println!("{:?}, {:?}: {} / {}", points[i], points[j], in_acute_part, is_interior[i]);

            if !((in_acute_part && is_interior[i]) || (!in_acute_part && !is_interior[i])) {
                println!("\tPassed");
                continue;
            }

            let br = (max(points[i].0, points[j].0), max(points[i].1, points[j].1));
            let tr = (max(points[i].0, points[j].0), min(points[i].1, points[j].1));
            let bl = (min(points[i].0, points[j].0), max(points[i].1, points[j].1));
            let tl = (min(points[i].0, points[j].0), min(points[i].1, points[j].1));

            // We check for the four edges our candidate if there exists an edge of the polygon
            // intersecting it
            let mut intersect = false;

            for k in 0..nb_points {
                let cur = points[k];
                let next = points[next_ind(k)];

                match orientations[k] {
                    Orientation::North | Orientation::South => {
                        // We need to check horizontal lines
                        if (bl.0 < cur.0 && cur.0 < br.0 && min(cur.1, next.1) < bl.1 && bl.1 <= max(cur.1, next.1)) ||
                            (tl.0 < cur.0 && cur.0 < tr.0 && min(cur.1, next.1) <= tl.1 && tl.1 < max(cur.1, next.1))
                        {
                            println!("\tIntersect: {:?}, {:?}", cur, next);
                            intersect = true;
                        }
                    }
                    Orientation::East | Orientation::West => {
                        // We need to check vertical lines
                        if (tl.1 < cur.1 && cur.1 < bl.1 && min(cur.0, next.0) <= tl.0 && tl.0 < max(cur.0, next.0)) ||
                            (tr.1 < cur.1 && cur.1 < br.1 && min(cur.0, next.0) < tr.0 && tr.0 <= max(cur.0, next.0))
                        {
                            println!("\tIntersect: {:?}, {:?}", cur, next);
                            intersect = true;
                        }
                    }
                }

                if intersect {
                    break;
                }
            }

            if !intersect {
                ans2 = max(ans2, get_area(&br, &tl));
                println!("\t{ans2}");
            }
        }
    }

    println!("First part answer: {ans1}");
    println!("Second part answer: {ans2}");

    Ok(())
}
