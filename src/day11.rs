use std::collections::HashMap;

// Returns a quadruplet (a, b, c, d) where paths a don't go through "dac" nor "fft", b goes through "dac"
// but not "fft", c the converse and d go throught both "dac" and "fft"
fn count_paths(graph: &HashMap<&str, Vec<&str>>, current: &&str, paths_count: &mut HashMap<&str, Option<(usize, usize, usize, usize)>>) -> (usize, usize, usize, usize) {
    if let Some(p) = paths_count[current] {
        p
    } else {
        let val = graph[current].iter().map(
            |next| { let (a, b, c, d) = count_paths(graph, next, paths_count);
            if current == &"dac" {
                (0, a + b, 0, c + d)
            } else if current == &"fft" {
                (0, 0, a + c, b + d)
            } else {
                (a, b, c, d)
            }
            }
            ).fold((0, 0, 0, 0), |x, y| (x.0 + y.0, x.1 + y.1, x.2 + y.2, x.3 + y.3));

        *paths_count.get_mut(*current).unwrap() = Some(val);

        val
    }
}

pub(crate) const DAY: usize = 11;
pub fn run(input: &str) -> Result<(), ()> {
    let mut graph = HashMap::new();

    for (node, outs) in input.trim().lines().map(|line| line.split_once(":").expect("Wrong format")) {
        graph.insert(node, outs.split_whitespace().collect::<Vec<_>>());
    }

    let mut paths_count = HashMap::new();

    for (node, next) in graph.iter() {
        paths_count.entry(*node).or_insert(None);

        for p in next {
            paths_count.entry(*p).or_insert(None);
        }
    }
    paths_count.entry("out").and_modify(|n| *n = Some((1, 0, 0, 0)));

    let ans = count_paths(&graph, &"you", &mut paths_count);

    let ans1 = ans.0 + ans.1 + ans.2 + ans.3;

    // Reset paths_count
    for (node, next) in graph.iter() {
        *paths_count.get_mut(*node).unwrap() = None;

        for p in next {
            *paths_count.get_mut(*p).unwrap() = None;
        }
    }
    paths_count.entry("out").and_modify(|n| *n = Some((1, 0, 0, 0)));

    let ans2 = count_paths(&graph, &"svr", &mut paths_count).3;

    println!("First part answer: {ans1}");
    println!("Second part answer: {ans2}");

    Ok(())
}
