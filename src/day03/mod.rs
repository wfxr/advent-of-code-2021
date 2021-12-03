use crate::{solution, Result};

fn part1(input: &str) -> Result<usize> {
    let cols = input.lines().next().ok_or("empty input")?.len();
    let mut v = vec![0; cols];
    let mut rows = 0;
    for line in input.lines() {
        rows += 1;
        for (col, c) in line.bytes().enumerate() {
            v[col] += (c - b'0') as usize
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    for &n in &v {
        gamma <<= 1;
        epsilon <<= 1;
        if n > rows / 2 {
            gamma |= 1;
        } else {
            epsilon |= 1;
        }
    }
    Ok(gamma * epsilon)
}

fn part2(_input: &str) -> Result<usize> {
    unimplemented!()
}

solution!(part1 => 841526, part2 => 0);
