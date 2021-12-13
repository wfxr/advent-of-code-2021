use crate::{solution, Result};
use std::{cell::Cell, collections::HashMap, rc::Rc};

const START: usize = 0;
const END: usize = START + 2;

fn parse_input(input: &str) -> Vec<(Vec<usize>, Rc<Cell<isize>>)> {
    let mut small = HashMap::from([("start", START), ("end", END)]);
    let mut next_small_id = (END + 2..).step_by(2);
    let mut big = HashMap::new();
    let mut next_big_id = (1..).step_by(2);
    input
        .lines()
        .filter_map(|line| {
            let mut iter = line.split('-').map(|s| match s.chars().any(|c| c.is_uppercase()) {
                true => *big.entry(s).or_insert_with(|| next_big_id.next().unwrap()),
                false => *small.entry(s).or_insert_with(|| next_small_id.next().unwrap()),
            });
            Some((iter.next()?, iter.next()?))
        })
        .flat_map(|(l, r)| [(l, r), (r, l)])
        .fold(Vec::new(), |mut acc, (from, to)| {
            (acc.len() <= from).then(|| acc.resize_with(from + 1, || (Vec::new(), Rc::default())));
            let (caves, count) = &mut acc[from];
            caves.push(to);
            count.set(if from % 2 == 1 { -1 } else { 1 });
            acc
        })
}

fn dfs(edges: &[(Vec<usize>, Rc<Cell<isize>>)], curr: usize, extra: usize) -> usize {
    let (nodes, count) = &edges[curr];
    let count = Rc::clone(count);
    match (curr, count.get(), extra) {
        (START | END, 0, _) | (_, 0, 0) => 0,
        (curr, n, mut extra) => {
            match n {
                0 => extra -= 1,
                _ => count.set(n - 1),
            }
            let paths = nodes.iter().map(|&x| dfs(edges, x, extra)).sum::<usize>();
            if n != 0 {
                count.set(n)
            }
            paths + if curr == END { 1 } else { 0 }
        }
    }
}

fn part1(input: &str) -> Result<usize> {
    Ok(dfs(&parse_input(input), START, 0))
}

fn part2(input: &str) -> Result<usize> {
    Ok(dfs(&parse_input(input), START, 1))
}

solution!(part1 => 3421, part2 => 84870);
