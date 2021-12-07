use crate::{solution, Result};

fn fuel_cost(input: &str, pos_fn: fn(&mut [u32]) -> u32, fuel_fn: fn(u32) -> u32) -> Result<u32> {
    let mut input: Vec<u32> = input.split(',').map(|x| Ok(x.trim().parse()?)).collect::<Result<_>>()?;
    let pos = pos_fn(&mut input);
    Ok(input.iter().fold(0, |acc, x| acc + fuel_fn(x.abs_diff(pos))))
}

fn part1(input: &str) -> Result<u32> {
    fuel_cost(
        input,
        |seq| {
            let (_, &mut median, _) = seq.select_nth_unstable(seq.len() / 2);
            median
        },
        |d| d,
    )
}

fn part2(input: &str) -> Result<u32> {
    fuel_cost(
        input,
        |seq| seq.iter().sum::<u32>() / seq.len() as u32, // don't ask me why
        |d| d * (d + 1) / 2,
    )
}

solution!(part1 => 340987, part2 => 96987874);
