#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TachyanManifold {
    Beam(u64),
    Spliter,
    Dot
}

impl TryInto<TachyanManifold> for char {
    type Error = &'static str;

    fn try_into(self: char) -> Result<TachyanManifold, Self::Error> {
        match self {
            '.' => Ok(TachyanManifold::Dot),
            'S' => Ok(TachyanManifold::Beam(1)),
            '^' => Ok(TachyanManifold::Spliter),
             _  => Err("Not a manifold character.")
        }
    }
}

impl TachyanManifold {
    fn is_beam(self: &Self) -> bool {
        match self {
            TachyanManifold::Beam(_) => true,
            _ => false
        }
    }

    fn add_beam(self: Self, beam: &Self) -> Self {
        match (self, beam) {
            (TachyanManifold::Beam(x), TachyanManifold::Beam(y)) => TachyanManifold::Beam(x + *y),
            (_, TachyanManifold::Beam(y)) => TachyanManifold::Beam(*y),
            (_, _) => self
        }
    }

    fn get_beam(self: &Self) -> Option<u64> {
        match self {
            TachyanManifold::Beam(x) => Some(*x),
            _ => None
        }
    }
}

pub(crate) const DAY: usize = 7;
pub fn run(input: &str) -> Result<(), ()> {
    let mut lines = input.trim().lines().map(|line| line.chars().map(|c| TryInto::<TachyanManifold>::try_into(c).expect("Uncorrect file")));
    let mut state = &mut lines.next().expect("Empty file").collect::<Vec<_>>();
    let mut buffer = &mut vec![TachyanManifold::Dot; state.len()];

    let mut ans1 = 0;

    for line in lines {
        buffer.fill(TachyanManifold::Dot);

        for (i, c) in line.enumerate() {
            if state[i].is_beam() {
                if c == TachyanManifold::Spliter {
                    buffer[i - 1] = buffer[i - 1].add_beam(&state[i]);
                    buffer[i + 1] = buffer[i + 1].add_beam(&state[i]);
                    buffer[i] = TachyanManifold::Dot;

                    ans1 += 1;
                } else {
                    buffer[i] = buffer[i].add_beam(&state[i]);
                }
            }
        }

        let tmp = buffer;
        buffer = state;
        state = tmp;
    }

    let ans2 = state.iter().fold(0, |acc, b| b.get_beam().map(|v| v + acc).or(Some(acc)).unwrap());

    println!("First part answer: {ans1}");
    println!("Second part answer: {ans2}");

    Ok(())
}
