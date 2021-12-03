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

fn part2(input: &str) -> Result<u32> {
    let seq: Vec<_> = input.lines().collect();
    let oxygen = life_support_rating(&seq, |len, ones| if ones * 2 >= len { b'1' } else { b'0' })?;
    let co2 = life_support_rating(&seq, |len, ones| if ones * 2 >= len { b'0' } else { b'1' })?;
    Ok(oxygen * co2)
}

fn life_support_rating<F>(seq: &[&str], target_fn: F) -> Result<u32>
where
    F: Fn(usize, usize) -> u8,
{
    let mut seq: Vec<_> = seq.iter().map(|line| line.as_bytes()).collect();
    let mut col = 0;
    while seq.len() > 1 {
        let mut ones = 0;
        for c in seq.iter().map(move |line| line[col]) {
            if c == b'1' {
                ones += 1;
            }
        }
        let target = target_fn(seq.len(), ones);
        seq = seq.into_iter().filter(|line| line[col] == target).collect();
        col += 1;
    }
    Ok(match seq.first() {
        Some(line) => u32::from_str_radix(std::str::from_utf8(line)?, 2)?,
        None => 0,
    })
}

solution!(part1 => 841526, part2 => 4790390);
