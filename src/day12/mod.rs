use crate::{solution, Result};
use std::{cell::Cell, collections::HashMap, rc::Rc};

type Edges = HashMap<Cave, (Vec<Cave>, Rc<Cell<isize>>)>;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Cave {
    Start,
    End,
    Big(u8),
    Small(u8),
}

fn parse_input(input: &str) -> Edges {
    let mut big = HashMap::new();
    let mut small = HashMap::new();
    let mut parse_cave = |s| match s {
        "start" => Cave::Start,
        "end" => Cave::End,
        _ if s.chars().any(|c| c.is_uppercase()) => {
            let next_id = big.len();
            let id = *big.entry(s).or_insert_with(|| next_id);
            Cave::Big(id as u8)
        }
        _ => {
            let next_id = small.len();
            let id = *small.entry(s).or_insert_with(|| next_id);
            Cave::Small(id as u8)
        }
    };
    input
        .lines()
        .filter_map(|line| line.split_once('-'))
        .map(|(l, r)| (parse_cave(l), parse_cave(r)))
        .flat_map(|(l, r)| [(l, r), (r, l)])
        .fold(HashMap::new(), |mut acc, (from, to)| {
            acc.entry(from)
                .or_insert_with_key(|from| {
                    let nodes = Vec::new();
                    let count = if matches!(from, Cave::Big(..)) { -1 } else { 1 };
                    (nodes, Rc::new(Cell::new(count)))
                })
                .0
                .push(to);
            acc
        })
}

fn part1(input: &str) -> Result<usize> {
    fn dfs(edges: &Edges, curr: &Cave) -> usize {
        let (nodes, count) = &edges[curr];
        let count = Rc::clone(count);
        match count.get() {
            0 => 0,
            n => {
                count.set(n - 1);
                let paths = nodes.iter().map(|x| dfs(edges, x)).sum::<usize>();
                count.set(n);
                paths + if curr == &Cave::End { 1 } else { 0 }
            }
        }
    }
    Ok(dfs(&parse_input(input), &Cave::Start))
}

fn part2(input: &str) -> Result<usize> {
    fn dfs(edges: &Edges, curr: &Cave, extra: usize) -> usize {
        let (nodes, count) = &edges[curr];
        let count = Rc::clone(count);
        match (curr, count.get(), extra) {
            (Cave::Start | Cave::End, 0, _) | (_, 0, 0) => 0,
            (_, n, mut extra) => {
                match n {
                    0 => extra -= 1,
                    _ => count.set(n - 1),
                }
                let paths = nodes.iter().map(|x| dfs(edges, x, extra)).sum::<usize>();
                if n != 0 {
                    count.set(n)
                }
                paths + if curr == &Cave::End { 1 } else { 0 }
            }
        }
    }
    Ok(dfs(&parse_input(input), &Cave::Start, 1))
}

solution!(part1 => 3421, part2 => 84870);
