use crate::{solution, Result};

fn part1(input: &str) -> Result<u32> {
    let input: Vec<u32> = input.split(',').map(|x| Ok(x.trim().parse()?)).collect::<Result<_>>()?;
    let max = *input.iter().max().unwrap_or(&0);
    Ok((0..=max)
        .map(|i| input.iter().fold(0, |acc, x| acc + x.abs_diff(i)))
        .min()
        .unwrap_or(0))
}

fn part2(_input: &str) -> Result<usize> {
    todo!()
}

solution!(part1 => 340987, part2 => todo!());
