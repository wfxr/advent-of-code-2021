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
    let mut counter: HashMap<_, _> = edges.keys().map(|k| (k.to_string(), 0)).collect();

    let mut paths = Vec::new();
    dfs(&edges, &mut counter, &mut paths, &mut vec![], "start");
    Ok(paths.len())
}

fn dfs(
    edges: &HashMap<&str, Vec<&str>>,
    counter: &mut HashMap<String, usize>,
    paths: &mut Vec<Vec<String>>,
    path: &mut Vec<String>,
    curr: &str,
) {
    match ((is_big_cave(curr)), counter[curr]) {
        (true, _) | (false, 0) => {
            path.push(curr.to_string());
            counter.get_mut(curr).map(|x| *x += 1);
            (curr == "end").then(|| paths.push(path.clone()));
            edges[curr].iter().for_each(|x| dfs(edges, counter, paths, path, *x));
            counter.get_mut(curr).map(|x| *x -= 1);
            path.pop();
        }
        _ => {}
    }
}

fn is_big_cave(s: &str) -> bool {
    s.chars().any(|c| c.is_ascii_uppercase())
}

fn part2(_input: &str) -> Result<usize> {
    todo!()
}

solution!(part1 => 3421, part2 => todo!());
