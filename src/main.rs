use aoc2021::{build_solutions, Result, Solution};
use std::time::{Duration, Instant};

fn main() {
    if let Err(e) = try_main() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let mut solutions = build_solutions();
    if let Some(idx) = std::env::args().nth(1) {
        solutions.retain(|k, _| *k == idx)
    }

    let mut total = Duration::from_secs(0);
    for (idx, Solution { part1, part2 }) in solutions {
        println!("[{}]", idx);

        let input = std::fs::read_to_string(format!("src/{}/input", idx))?;

        let (t, result) = measure(|| part1(&input));
        total += t;
        println!("part 1: {:10} - time: {:?}", result?, t);

        let (t, result) = measure(|| part2(&input));
        total += t;
        println!("part 2: {:10} - time: {:?}", result?, t);

        println!();
    }

    println!("Total time: {:?}", total);

    Ok(())
}

fn measure<T, F>(f: F) -> (Duration, T)
where
    F: FnOnce() -> T,
{
    let now = Instant::now();
    let r = f();
    (now.elapsed(), r)
}
