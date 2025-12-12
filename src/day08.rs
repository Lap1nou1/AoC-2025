use regex::Regex;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use std::ops::Sub;

struct UnionFind<'a, T: Eq + Hash> {
    classes: HashMap<&'a T, (Option<&'a T>, usize)>,
}

impl<'a, T: Eq + Hash> UnionFind<'a, T> {
    fn new() -> Self {
        UnionFind {
            classes: HashMap::new(),
        }
    }
}

impl<'a, T: Eq + Hash + Clone> UnionFind<'a, T> {
    fn get_class_representent(self: &mut Self, x: &'a T) -> &'a T {
        if self.classes.contains_key(x) {
            if let Some(&(Some(px), sx)) = self.classes.get(x) {
                let rep = self.get_class_representent(px);

                *self.classes.get_mut(x).unwrap() = (Some(rep), sx);

                rep
            } else {
                x
            }
        } else {
            self.classes.insert(x, (None, 1));

            x
        }
    }

    fn share_same_class(self: &mut Self, x: &'a T, y: &'a T) -> bool {
        let xp = self.get_class_representent(x);
        let yp = self.get_class_representent(y);

        xp == yp
    }

    fn union(self: &mut Self, x: &'a T, y: &'a T) -> bool {
        let px = self.get_class_representent(&x);
        let py = self.get_class_representent(&y);

        if px != py {
            let sx = self.classes[px].1;
            let sy = self.classes[py].1;

            if sx < sy {
                self.classes.insert(&px, (Some(py), sx));
                self.classes.insert(&py, (None, sx + sy));
            } else {
                self.classes.insert(&px, (None, sx + sy));
                self.classes.insert(&py, (Some(px), sy));
            }

            true
        } else {
            false
        }
    }

    fn get_nth_biggest(self: &Self, n: usize) -> Vec<usize> {
        let mut sizes = vec![0; n];

        for (p, s) in self.classes.values() {
            if p.is_none() {
                let part = sizes.partition_point(|x| x > s);

                if part < sizes.len() {
                    for i in (part..sizes.len() - 1).rev() {
                        sizes[i + 1] = sizes[i]
                    }

                    sizes[part] = *s;
                }
            }
        }

        sizes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vec3D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Sub for Vec3D {
    type Output = Vec3D;

    fn sub(self: Self, rhs: Self) -> Self {
        Vec3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Vec3D {
    fn new(x: i64, y: i64, z: i64) -> Vec3D {
        Vec3D { x, y, z }
    }

    fn squared_norm(self: &Self) -> i64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

pub(crate) const DAY: usize = 8;
pub fn run(input: &str) -> Result<(), ()> {
    let re = Regex::new(r"(?<x>\d+),(?<y>\d+),(?<z>\d+)").unwrap();
    let junctions = input
        .lines()
        .map(|line| {
            let capt = re.captures(line).expect("Wrong format");
            Vec3D::new(
                capt["x"].parse::<i64>().unwrap(),
                capt["y"].parse::<i64>().unwrap(),
                capt["z"].parse::<i64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut heap = BinaryHeap::new();
    let nb_cables = 10;

    for i in 0..junctions.len() {
        for j in i + 1..junctions.len() {
            heap.push(Reverse((
                Vec3D::squared_norm(&(junctions[i] - junctions[j])),
                &junctions[i],
                &junctions[j],
            )))
        }
    }

    let mut uf = UnionFind::new();
    let mut count = 0;

    for _ in 0..nb_cables {
            let Reverse((d, i, j)) = heap.pop().expect("Not enough junctions");

            if uf.union(i, j) {
                count += 1
            }
    }

    let ans1 = uf.get_nth_biggest(3).iter().fold(1, |x, y| x * y);

    let mut ans2 = 0;

    loop {
        let Reverse((d, i, j)) = heap.pop().expect("Not enough junctions");

        if uf.union(i, j) {
            count += 1
        }

        if count == 999 {
            ans2 = i.x * j.x;

            break
        }
    }

    println!("First part answer: {ans1}");
    println!("Second part answer: {ans2}");

    Ok(())
}
