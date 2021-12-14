use crate::{solution, Result};

fn solve(input: &str, loops: usize) -> Result<usize> {
    const N: usize = 26;
    let id = |b| (b - b'A') as usize;
    let left = |i| i / N;
    let right = |i| i % N;

    let (init, rules) = input.split_once("\n\n").ok_or("missing rules")?;
    let init = init.bytes().map(id).collect::<Vec<_>>();
    let trans = rules
        .lines()
        .map(|l| l.as_bytes())
        .filter_map(|l| Some((l.get(0)?, l.get(1)?, l.get(6)?)))
        .map(|(&a, &b, &v)| (id(a) * N + id(b), id(v)))
        .fold([0; N * N], |mut acc, (k, v)| {
            acc[k] = v;
            acc
        });

    let mut curr = [0; N * N];
    init.array_windows().map(|&[a, b]| a * N + b).for_each(|x| curr[x] += 1);

    (0..loops).for_each(|_| {
        let mut next = [0; N * N];
        curr.iter().enumerate().filter(|(_, &n)| n > 0).for_each(|(i, &n)| {
            next[left(i) * N + trans[i]] += n;
            next[trans[i] * N + right(i)] += n;
        });
        curr = next;
    });

    match init[..] {
        [first, .., last] => {
            let mut counter = [0; N];
            curr.iter().enumerate().filter(|(_, &n)| n > 0).for_each(|(i, &x)| {
                counter[left(i)] += x;
                counter[right(i)] += x;
            });
            counter[first] += 1;
            counter[last] += 1;
            let (min, max) = counter
                .iter()
                .filter(|&&x| x > 0)
                .fold((usize::MAX, 0), |(min, max), &x| (min.min(x), max.max(x)));
            Ok((max - min) / 2)
        }
        _ => Ok(0),
    }
}

fn part1(input: &str) -> Result<usize> {
    solve(input, 10)
}

fn part2(input: &str) -> Result<usize> {
    solve(input, 40)
}

solution!(part1 => 3247, part2 => 4110568157153);
