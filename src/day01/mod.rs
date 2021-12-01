use crate::{solution, Result};

fn parse_input(input: &str) -> Result<Vec<u32>> {
    input.split_ascii_whitespace().map(|s| Ok(s.parse()?)).collect()
}

fn part1(input: &str) -> Result<usize> {
    let seq = parse_input(input)?;
    Ok(seq.windows(2).filter(|w| w[1] > w[0]).count())
}

fn part2(input: &str) -> Result<usize> {
    let seq = parse_input(input)?;
    Ok(seq.windows(4).filter(|w| w[3] > w[0]).count())
}

solution!(part1 => 1390, part2 => 1457);
