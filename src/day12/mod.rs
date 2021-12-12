use std::collections::HashMap;

use crate::{solution, Result};

fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .filter_map(|line| line.split_once('-'))
        .flat_map(|(l, r)| [(l, r), (r, l)])
        .fold(HashMap::new(), |mut acc, (from, to)| {
            acc.entry(from).or_insert(vec![]).push(to);
            acc
        })
}

fn part1(input: &str) -> Result<usize> {
    let edges = parse_input(input);
    let mut counter: HashMap<_, _> = edges
        .keys()
        .map(|&k| {
            (k, match k {
                k if k.chars().any(|c| c.is_ascii_uppercase()) => -1,
                _ => 1,
            })
        })
        .collect();
    Ok(dfs(&edges, &mut counter, "start"))
}

fn dfs(edges: &HashMap<&str, Vec<&str>>, counter: &mut HashMap<&str, isize>, curr: &str) -> usize {
    match counter[curr] {
        0 => 0,
        _ => {
            counter.get_mut(curr).map(|x| *x -= 1);
            let count = edges[curr].iter().map(|x| dfs(edges, counter, *x)).sum::<usize>();
            counter.get_mut(curr).map(|x| *x += 1);
            count
                + match curr {
                    "end" => 1,
                    _ => 0,
                }
        }
    }
}

fn part2(input: &str) -> Result<usize> {
    let edges = parse_input(input);
    let mut counter: HashMap<_, _> = edges
        .keys()
        .map(|&k| {
            (k, match k {
                k if k.chars().any(|c| c.is_ascii_uppercase()) => -1,
                _ => 1,
            })
        })
        .collect();
    Ok(dfs2(&edges, &mut counter, "start", &mut 1))
}

fn dfs2(edges: &HashMap<&str, Vec<&str>>, counter: &mut HashMap<&str, isize>, curr: &str, extra: &mut usize) -> usize {
    match (curr, counter[curr], *extra) {
        ("start" | "end", 0, _) => 0,
        (_, 0, 0) => 0,
        (_, n, _) => {
            if n == 0 {
                *extra -= 1
            } else {
                counter.get_mut(curr).map(|x| *x -= 1);
            }
            let count = edges[curr]
                .iter()
                .map(|x| dfs2(edges, counter, *x, extra))
                .sum::<usize>();
            if n == 0 {
                *extra += 1;
            } else {
                counter.get_mut(curr).map(|x| *x += 1);
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
