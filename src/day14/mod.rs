use crate::{solution, Result};

fn solve(input: &str, loops: usize) -> Result<usize> {
    let id = |b| (b - b'A') as usize;
    let (template, rules) = input.split_once("\n\n").ok_or("missing rules")?;
    let template = template.bytes().map(id).collect::<Vec<_>>();
    let rules = rules
        .lines()
        .map(|l| l.as_bytes())
        .filter_map(|l| Some((l.get(0)?, l.get(1)?, l.get(6)?)))
        .map(|(&a, &b, &v)| (id(a) * N + id(b), id(v)))
        .fold([0; N * N], |mut acc, (k, v)| {
            acc[k] = v;
            acc
        });
    let mut pairs = [0; N * N];
    template
        .array_windows()
        .map(|&[a, b]| a * N + b)
        .for_each(|x| pairs[x] += 1);

    for _ in 0..loops {
        let mut new_pairs = [0; N * N];
        for (i, &x) in pairs.iter().enumerate() {
            new_pairs[i / N * N + rules[i]] += x;
            new_pairs[rules[i] * N + i % N] += x;
        }
        pairs = new_pairs;
    }

    let mut counter = [0; N];
    pairs.iter().enumerate().for_each(|(i, &x)| {
        counter[i / N] += x;
        counter[i % N] += x;
    });
    counter[*template.first().unwrap()] += 1;
    counter[*template.last().unwrap()] += 1;
    let (min, max) = counter
        .iter()
        .filter(|&&x| x > 0)
        .fold((usize::MAX, 0), |(min, max), &x| (min.min(x), max.max(x)));
    Ok((max - min) / 2)
}

fn part1(input: &str) -> Result<usize> {
    solve(input, 10)
}

const N: usize = 26;

fn part2(input: &str) -> Result<usize> {
    solve(input, 40)
}

solution!(part1 => 3247, part2 => 4110568157153);
