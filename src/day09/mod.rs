use std::{cmp::Reverse, collections::BinaryHeap};

use crate::{solution, Result};

#[rustfmt::skip]
fn part1(input: &str) -> Result<u32> {
    let m: Vec<_> = input.lines().map(|l| l.as_bytes()).collect();
    Ok((0..m.len())
        .flat_map(|i| (0..m[i].len()).map(move |j| (i, j)))
        .filter(|&(i, j)| {
            (i == 0              || m[i][j] < m[i - 1][j]) &&
            (j == 0              || m[i][j] < m[i][j - 1]) &&
            (i == m.len() - 1    || m[i][j] < m[i + 1][j]) &&
            (j == m[i].len() - 1 || m[i][j] < m[i][j + 1])
        })
        .map(|(i, j)| (m[i][j] - b'0' + 1) as u32)
        .sum())
}

#[rustfmt::skip]
fn part2(input: &str) -> Result<usize> {
    fn mark_basin(m: &mut [Vec<u8>], i: usize, j: usize) -> usize {
        match m[i][j] {
            b'9' => 0,
            _ => {
                m[i][j] = b'9';
                let mut size = 1;
                if i > 0              { size += mark_basin(m, i - 1, j) }
                if j > 0              { size += mark_basin(m, i, j - 1) }
                if i < m.len() - 1    { size += mark_basin(m, i + 1, j) }
                if j < m[i].len() - 1 { size += mark_basin(m, i, j + 1) }
                size
            }
        }
    }

    let mut m: Vec<Vec<_>> = input.lines().map(|l| l.bytes().collect()).collect();
    let mut topk = BinaryHeap::new();
    for i in 0..m.len() {
        for j in 0..m[i].len() {
            topk.push(Reverse(mark_basin(&mut m, i, j)));
            if topk.len() > 3 {
                topk.pop();
            }
        }
    }
    Ok(topk.into_iter().map(|Reverse(x)| x).product())
}

solution!(part1 => 514, part2 => 1103130);
