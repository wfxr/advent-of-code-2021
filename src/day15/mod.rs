use crate::{solution, Result};
use std::{cmp::Reverse, collections::BinaryHeap};

type Map<const N: usize> = [[u8; N]; N];

fn parse_input<const N: usize>(input: &str) -> Result<Map<N>> {
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
        .map_err(|_| "invalid input".into())
}

fn part1(input: &str) -> Result<u32> {
    Ok(shortest_path(&mut parse_input::<100>(input)?))
}

fn part2(_input: &str) -> Result<usize> {
    todo!()
}

fn shortest_path<const N: usize>(m: &mut Map<N>) -> u32 {
    let mut heap = BinaryHeap::from([(Reverse(0), (0, 0))]);
    while let Some((Reverse(risk), (i, j))) = heap.pop() {
        if (i, j) == (N - 1, N - 1) {
            return risk;
        }
        [(i.wrapping_sub(1), j), (i + 1, j), (i, j.wrapping_sub(1)), (i, j + 1)]
            .into_iter()
            .filter(|&(i, j)| i < N && j < N)
            .for_each(|(i, j)| {
                if m[i][j] > 0 {
                    heap.push((Reverse(risk + m[i][j] as u32), (i, j)));
                    m[i][j] = 0;
                }
            })
    }
    0
}

solution!(part1 => 462, part2 => todo!());
