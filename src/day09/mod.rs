use crate::{solution, Result};
use std::iter::once;

fn parse_input(input: &str) -> Result<Vec<Vec<u8>>> {
    let len = input.lines().next().ok_or("empty input")?.len();
    let mut rs = vec![vec![9; len + 2]];
    for line in input.lines() {
        rs.push(once(9).chain(line.bytes().map(|b| b - b'0')).chain(once(9)).collect())
    }
    rs.push(vec![9; len + 2]);
    Ok(rs)
}

fn part1(input: &str) -> Result<u32> {
    let m = parse_input(input)?;
    Ok(m.iter()
        .enumerate()
        .skip(1)
        .take(m.len() - 2)
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .skip(1)
                .take(row.len() - 1)
                .filter_map(|(j, &x)| {
                    if x < m[i - 1][j] && x < m[i + 1][j] && x < m[i][j - 1] && x < m[i][j + 1] {
                        Some(x as u32 + 1)
                    } else {
                        None
                    }
                })
                .sum::<u32>()
        })
        .sum())
}

fn part2(_input: &str) -> Result<usize> {
    todo!()
}

solution!(part1 => 514, part2 => todo!());
