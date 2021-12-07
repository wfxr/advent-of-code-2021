use crate::{solution, Result};

fn fuel_cost(input: &str, f: fn(u32) -> u32) -> Result<u32> {
    let input: Vec<u32> = input.split(',').map(|x| Ok(x.trim().parse()?)).collect::<Result<_>>()?;
    let max = *input.iter().max().unwrap_or(&0);
    Ok((0..=max)
        .map(|i| input.iter().fold(0, |acc, x| acc + f(x.abs_diff(i))))
        .min()
        .unwrap_or(0))
}

fn part1(input: &str) -> Result<u32> {
    fuel_cost(input, |dis| dis)
}

fn part2(input: &str) -> Result<u32> {
    fuel_cost(input, |dis| dis * (dis + 1) / 2)
}

solution!(part1 => 340987, part2 => 96987874);
