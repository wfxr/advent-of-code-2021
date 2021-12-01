use crate::{solution, Result};

fn parse_input(input: &str) -> Result<Vec<u32>> {
    input.split_ascii_whitespace().map(|s| Ok(s.parse()?)).collect()
}

fn increased_count(seq: &[u32]) -> usize {
    seq.windows(2).filter(|w| w[1] > w[0]).count()
}

fn part1(input: &str) -> Result<usize> {
    let seq = parse_input(input)?;
    Ok(increased_count(&seq))
}

fn part2(input: &str) -> Result<usize> {
    let seq = parse_input(input)?;
    let wins: Vec<u32> = seq.windows(3).map(|w| w.iter().sum()).collect();
    Ok(increased_count(&wins))
}

solution!(part1 => 0, part2 => 0);
