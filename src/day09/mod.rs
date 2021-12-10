use std::{cmp::Reverse, collections::BinaryHeap};

use crate::{solution, Result};

fn neighbors((i, j): (usize, usize), (maxi, maxj): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    [(i, j.wrapping_sub(1)), (i, j + 1), (i.wrapping_sub(1), j), (i + 1, j)]
        .into_iter()
        .filter(move |&(ni, nj)| ni < maxi && nj < maxj)
}

fn part1(input: &str) -> Result<u32> {
    let m: Vec<_> = input.lines().map(|l| l.as_bytes()).collect();
    Ok((0..m.len())
        .flat_map(|i| (0..m[i].len()).map(move |j| (i, j)))
        .filter(|&(i, j)| neighbors((i, j), (m.len(), m[i].len())).all(|(ni, nj)| m[i][j] < m[ni][nj]))
        .map(|(i, j)| (m[i][j] - b'0' + 1) as u32)
        .sum())
}

fn part2(input: &str) -> Result<usize> {
    fn mark_basin(m: &mut [Vec<u8>], i: usize, j: usize) -> usize {
        match &mut m[i][j] {
            b'9' => 0,
            x => {
                *x = b'9';
                1 + neighbors((i, j), (m.len(), m[i].len()))
                    .map(|(ni, nj)| mark_basin(m, ni, nj))
                    .sum::<usize>()
            }
        }
    }

    let mut m: Vec<Vec<_>> = input.lines().map(|l| l.bytes().collect()).collect();
    let mut topk = BinaryHeap::new();
    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] < b'9' {
                topk.push(Reverse(mark_basin(&mut m, i, j)));
                if topk.len() > 3 {
                    topk.pop();
                }
            }
        }
    }
    Ok(topk.into_iter().map(|Reverse(x)| x).product())
}

solution!(part1 => 514, part2 => 1103130);
