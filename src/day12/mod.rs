use crate::{solution, Result};
use std::collections::HashMap;

const START: usize = 0;
const END: usize = 1;

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    let m = input.lines().flat_map(|line| line.split('-')).fold(
        HashMap::from([("start", START), ("end", END)]),
        |mut acc, cave| {
            let id = acc.len();
            acc.entry(cave).or_insert(id);
            acc
        },
    );
    let mut rs: Vec<Vec<usize>> = input
        .lines()
        .filter_map(|line| line.split_once('-').map(|(l, r)| (m[l], m[r])))
        .flat_map(|(l, r)| [(l, r), (r, l)])
        .fold(Vec::new(), |mut acc, (l, r)| {
            (acc.len() <= l).then(|| acc.resize(l + 1, Vec::new()));
            acc[l].push(r);
            acc
        });
    m.iter()
        .filter(|&(name, _)| name.chars().any(|c| c.is_uppercase()))
        .for_each(|(_, &x)| {
            let smalls = rs[x].clone();
            for nodes in rs.iter_mut() {
                if nodes.contains(&x) {
                    nodes.drain_filter(|s| s == &x);
                    nodes.extend(smalls.iter());
                }
            }
        });
    rs
}

fn dfs(edges: &[Vec<usize>], visited: &mut Vec<bool>, curr: usize, extra: usize) -> usize {
    edges[curr].iter().fold(0, |acc, &next| match next {
        next if next == START => acc,
        next if next == END => acc + 1,
        next if visited[next] && extra == 0 => acc,
        next if !visited[next] => {
            visited[next] = true;
            let paths = dfs(edges, visited, next, extra);
            visited[next] = false;
            acc + paths
        }
        next => acc + dfs(edges, visited, next, extra - 1),
    })
}

fn part1(input: &str) -> Result<usize> {
    let edges = parse_input(input);
    Ok(dfs(&edges, &mut vec![false; edges.len()], START, 0))
}

fn part2(input: &str) -> Result<usize> {
    let edges = parse_input(input);
    Ok(dfs(&edges, &mut vec![false; edges.len()], START, 1))
}

solution!(part1 => 3421, part2 => 84870);
