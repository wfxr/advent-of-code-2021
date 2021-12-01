use aoc2021::{build_solutions, Result, Solution};
use std::time::{Duration, Instant};

fn main() {
    if let Err(e) = || -> Result<_> {
        let index = std::env::args().nth(1).ok_or("missing solution index")?;

        let solutions = build_solutions();
        let mut solutions: Vec<(&str, &Solution)> = match index.as_str() {
            "all" => solutions.iter().map(|(k, v)| (k.as_str(), v)).collect(),
            _ => vec![(&index, solutions.get(&index).ok_or("solution not found")?)],
        };

        solutions.sort_unstable_by_key(|&(k, _)| k);

        let mut total = Duration::from_secs(0);
        for (index, Solution { part1, part2 }) in solutions {
            println!("[{}]", index);

            let input = std::fs::read_to_string(format!("src/{}/input", index))?;

            let (t, result) = measure(|| part1(&input));
            total += t;
            println!("part 1: {} - time: {:?}", result?, t);

            let (t, result) = measure(|| part2(&input));
            total += t;
            println!("part 2: {} - time: {:?}", result?, t);

            println!();
        }

        println!("Total time: {:?}", total);

        Ok(())
    }() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn measure<T, F>(f: F) -> (Duration, T)
where
    F: FnOnce() -> T,
{
    let now = Instant::now();
    let r = f();
    (now.elapsed(), r)
}
