use crate::{solution, Result};
use std::collections::HashMap;

const START: usize = 0;
const END: usize = 1;

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    let id = input.lines().flat_map(|line| line.split('-')).fold(
        HashMap::from([("start", START), ("end", END)]),
        |mut acc, cave| {
            let id = acc.len();
            acc.entry(cave).or_insert(id);
            acc
        },
    );

    let mut caves = vec![vec![]; id.len()];
    input
        .lines()
        .filter_map(|line| line.split_once('-').map(|(l, r)| (id[l], id[r])))
        .flat_map(|(l, r)| [(l, r), (r, l)])
        .filter(|(l, r)| l != &END && r != &START)
        .for_each(|(l, r)| caves[l].push(r));

    // flatten big caves
    id.iter()
        .filter(|&(name, _)| name.chars().any(|c| c.is_uppercase()))
        .for_each(|(_, &x)| {
            let smalls = caves[x].clone();
            caves.iter_mut().filter(|nodes| nodes.contains(&x)).for_each(|nodes| {
                nodes.drain_filter(|s| s == &x);
                nodes.extend(smalls.iter());
            });
        });
    caves
}

fn dfs(caves: &[Vec<usize>], visited: &mut Vec<bool>, curr: usize, extra: usize) -> usize {
    caves[curr].iter().fold(0, |acc, &next| match next {
        END => acc + 1,
        _ => match visited[next] {
            true => match extra {
                0 => acc,
                _ => acc + dfs(caves, visited, next, extra - 1),
            },
            _ => {
                visited[next] = true;
                let paths = dfs(caves, visited, next, extra);
                visited[next] = false;
                acc + paths
            }
        },
    })
}

fn part1(input: &str) -> Result<usize> {
    let caves = parse_input(input);
    Ok(dfs(&caves, &mut vec![false; caves.len()], START, 0))
}

fn part2(input: &str) -> Result<usize> {
    let caves = parse_input(input);
    Ok(dfs(&caves, &mut vec![false; caves.len()], START, 1))
}

solution!(part1 => 3421, part2 => 84870);
