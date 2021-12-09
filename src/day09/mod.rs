use crate::{solution, Result};

fn part1(input: &str) -> Result<u32> {
    let m: Vec<_> = input.lines().map(|l| l.as_bytes()).collect();
    Ok(m.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(j, &x)| {
                    (i == 0 || x < m[i - 1][j])
                        && (i == m.len() - 1 || x < m[i + 1][j])
                        && (j == 0 || x < m[i][j - 1])
                        && (j == row.len() - 1 || x < m[i][j + 1])
                })
                .map(|(_, &x)| (x - b'0' + 1) as u32)
                .sum::<u32>()
        })
        .sum())
}

fn part2(input: &str) -> Result<usize> {
    let mut m: Vec<Vec<_>> = input.lines().map(|l| l.bytes().collect()).collect();
    let mut basins = Vec::new();

    fn mark_basin(m: &mut [Vec<u8>], i: usize, j: usize) -> usize {
        match m[i][j] {
            b'9' => 0,
            _ => {
                let mut size = 1;
                m[i][j] = b'9';
                if i > 0 {
                    size += mark_basin(m, i - 1, j);
                }
                if j > 0 {
                    size += mark_basin(m, i, j - 1);
                }
                if i < m.len() - 1 {
                    size += mark_basin(m, i + 1, j)
                }
                if j < m[i].len() - 1 {
                    size += mark_basin(m, i, j + 1);
                }
                size
            }
        }
    }

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            let size = mark_basin(&mut m, i, j);
            if size != 0 {
                basins.push(size)
            }
        }
    }

    basins.sort_unstable();
    Ok(basins.iter().rev().take(3).product())
}

solution!(part1 => 514, part2 => 1103130);
