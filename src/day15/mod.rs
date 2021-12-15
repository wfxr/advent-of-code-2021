use std::{cmp::Reverse, collections::BinaryHeap};

use crate::{solution, Result};

fn parse_input(input: &str) -> (Vec<u8>, (usize, usize)) {
    let height = input.lines().count();
    let width = input.lines().next().map(|l| l.len()).unwrap_or(0);
    let input = input.lines().flat_map(|l| l.bytes().map(|c| c - b'0')).collect();
    (input, (height, width))
}

fn part1(input: &str) -> Result<u32> {
    let (input, (height, width)) = parse_input(input);
    Ok(shortest_path(&input, height, width))
}

fn part2(_input: &str) -> Result<usize> {
    todo!()
}

fn shortest_path(m: &[u8], h: usize, w: usize) -> u32 {
    let mut dist: Vec<_> = (0..h * w).map(|_| u32::MAX).collect();
    let mut heap = BinaryHeap::new();

    dist[0] = 0;
    heap.push((Reverse(0), 0));

    let neighbors = |pos: usize| {
        let (i, j) = (pos / h, pos % h);
        [(i.wrapping_sub(1), j), (i + 1, j), (i, j.wrapping_sub(1)), (i, j + 1)]
            .into_iter()
            .filter_map(|(i, j)| (i < h && j < w).then(|| i * w + j))
    };
    while let Some((Reverse(risk), pos)) = heap.pop() {
        if pos == h * w - 1 {
            return risk;
        }
        if risk > dist[pos] {
            continue;
        }
        neighbors(pos).for_each(|pos| {
            let risk = risk + m[pos] as u32;
            if risk < dist[pos] {
                heap.push((Reverse(risk), pos));
                dist[pos] = risk;
            }
        })
    }
    0
}

solution!(part1 => 462, part2 => todo!());
