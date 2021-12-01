use crate::{solution, Result};

fn parse_input(input: &str) -> Result<Vec<u32>> {
    input.split_ascii_whitespace().map(|s| Ok(s.parse()?)).collect()
}

fn part1(input: &str) -> Result<usize> {
    let seq = parse_input(input)?;
    Ok(seq
        .iter()
        .skip(1)
        .zip(seq.iter())
        .filter(|&(curr, prev)| curr > prev)
        .count())
}

fn part2(input: &str) -> Result<usize> {
    unimplemented!()
}

solution!(part1 => 0, part2 => 0);
