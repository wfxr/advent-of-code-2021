use std::collections::HashMap;

use crate::{solution, Result};

type Edges<'a> = HashMap<&'a str, Vec<&'a str>>;
type Counter<'a> = HashMap<&'a str, isize>;

fn parse_input(input: &str) -> (Edges, Counter) {
    let edges = input
        .lines()
        .filter_map(|line| line.split_once('-'))
        .flat_map(|(l, r)| [(l, r), (r, l)])
        .fold(HashMap::new(), |mut acc, (from, to)| {
            acc.entry(from).or_insert(vec![]).push(to);
            acc
        });
    let counter: HashMap<_, _> = edges
        .keys()
        .map(|&k| {
            (k, match k {
                k if k.chars().any(|c| c.is_ascii_uppercase()) => -1,
                _ => 1,
            })
        })
        .collect();
    (edges, counter)
}

fn part1(input: &str) -> Result<usize> {
    let (edges, mut counter) = parse_input(input);
    Ok(dfs(&edges, &mut counter, "start"))
}

fn dfs(edges: &Edges, counter: &mut Counter, curr: &str) -> usize {
    match counter[curr] {
        0 => 0,
        _ => {
            if let Some(n) = counter.get_mut(curr) {
                *n -= 1
            }
            let count = edges[curr].iter().map(|x| dfs(edges, counter, *x)).sum::<usize>();
            if let Some(n) = counter.get_mut(curr) {
                *n += 1
            }
            count
                + match curr {
                    "end" => 1,
                    _ => 0,
                }
        }
    }
}

fn part2(input: &str) -> Result<usize> {
    let (edges, mut counter) = parse_input(input);
    Ok(dfs2(&edges, &mut counter, "start", 1))
}

fn dfs2(edges: &Edges, counter: &mut Counter, curr: &str, extra: usize) -> usize {
    match (curr, counter[curr], extra) {
        ("start" | "end", 0, _) => 0,
        (_, 0, 0) => 0,
        (_, n, extra) => {
            if n != 0 {
                if let Some(n) = counter.get_mut(curr) {
                    *n -= 1
                }
            }
            let count = edges[curr]
                .iter()
                .map(|x| dfs2(edges, counter, *x, if n == 0 { extra - 1 } else { extra }))
                .sum::<usize>();
            if n != 0 {
                if let Some(n) = counter.get_mut(curr) {
                    *n += 1
                }
            }
            count
                + match curr {
                    "end" => 1,
                    _ => 0,
                }
        }
    }
}

solution!(part1 => 3421, part2 => 84870);
