use crate::{solution, Result};
use std::{cmp::Reverse, collections::BinaryHeap};

const N: usize = 100;

fn parse_input<const N: usize>(input: &str) -> Result<[[u8; N]; N]> {
    input
        .lines()
        .map(|l| {
            l.bytes()
                .map(|c| c - b'0')
                .collect::<Vec<_>>()
                .try_into()
                .map_err(|_| "invalid row".into())
        })
        .collect::<Result<Vec<_>>>()?
        .try_into()
        .map_err(|_| "invalid map".into())
}

fn lowest_risk<const N: usize>(m: &mut [[u8; N]; N]) -> u32 {
    let mut h = BinaryHeap::from([(Reverse(0), (0, 0))]);
    while let Some((Reverse(risk), (i, j))) = h.pop() {
        if (i, j) == (N - 1, N - 1) {
            return risk;
        }
        [(i.wrapping_sub(1), j), (i + 1, j), (i, j.wrapping_sub(1)), (i, j + 1)]
            .into_iter()
            .filter(|&(i, j)| i < N && j < N)
            .for_each(|(i, j)| {
                if m[i][j] > 0 {
                    h.push((Reverse(risk + m[i][j] as u32), (i, j)));
                    m[i][j] = 0;
                }
            })
    }
    0
}

fn part1(input: &str) -> Result<u32> {
    Ok(lowest_risk(&mut parse_input::<N>(input)?))
}

fn part2(input: &str) -> Result<u32> {
    const SCALE: usize = 5;
    let small = parse_input::<N>(input)?;
    let mut large = [[0; SCALE * N]; SCALE * N];
    (0..N * SCALE)
        .flat_map(|i| (0..N * SCALE).map(move |j| (i, j, i / N + j / N)))
        .for_each(|(i, j, k)| large[i][j] = ((small[i % N][j % N] as usize + k - 1) % 9) as u8 + 1);
    Ok(lowest_risk(&mut large))
}

solution!(part1 => 462, part2 => 2846);
