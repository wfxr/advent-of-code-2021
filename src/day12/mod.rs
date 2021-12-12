use crate::{solution, Result};
use std::{cell::Cell, collections::HashMap, rc::Rc};

type Edges = Vec<(Vec<usize>, Rc<Cell<isize>>)>;

const END: usize = 0;
const START: usize = 2;

fn parse_input(input: &str) -> Edges {
    let odd = &mut (1..).step_by(2);
    let even = &mut (4..).step_by(2);
    let mut big = HashMap::new();
    let mut small = HashMap::new();
    let mut parse_cave = |s| match s {
        "start" => START,
        "end" => END,
        _ if s.chars().any(|c| c.is_uppercase()) => *big.entry(s).or_insert_with(|| odd.next().unwrap()),
        _ => *small.entry(s).or_insert_with(|| even.next().unwrap()),
    };
    input
        .lines()
        .filter_map(|line| line.split_once('-'))
        .map(|(l, r)| (parse_cave(l), parse_cave(r)))
        .flat_map(|(l, r)| [(l, r), (r, l)])
        .fold(vec![(vec![], Rc::default()); 32], |mut acc, (from, to)| {
            let (caves, count) = acc.get_mut(from).unwrap();
            caves.push(to);
            *count = Rc::new(Cell::new(if from % 2 == 1 { -1 } else { 1 }));
            acc
        })
}

fn dfs(edges: &Edges, curr: usize, extra: usize) -> usize {
    let (nodes, count) = &edges[curr];
    let count = Rc::clone(count);
    match (curr, count.get(), extra) {
        (START | END, 0, _) | (_, 0, 0) => 0,
        (_, n, mut extra) => {
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
