use crate::{solution, Result};
use std::{cell::Cell, collections::HashMap, rc::Rc};

type Edges<'a> = HashMap<&'a str, (Vec<&'a str>, Rc<Cell<isize>>)>;

fn parse_input(input: &str) -> Edges {
    input
        .lines()
        .filter_map(|line| line.split_once('-'))
        .flat_map(|(l, r)| [(l, r), (r, l)])
        .fold(HashMap::new(), |mut acc, (from, to)| {
            acc.entry(from)
                .or_insert_with_key(|from| {
                    let nodes = vec![];
                    let count = match from {
                        from if from.chars().any(|c| c.is_ascii_uppercase()) => -1,
                        _ => 1,
                    };
                    (nodes, Rc::new(Cell::new(count)))
                })
                .0
                .push(to);
            acc
        })
}

fn part1(input: &str) -> Result<usize> {
    fn dfs(edges: &Edges, curr: &str) -> usize {
        let (nodes, count) = &edges[curr];
        let count = Rc::clone(count);
        match count.get() {
            0 => 0,
            n => {
                count.set(n - 1);
                let paths = nodes.iter().map(|x| dfs(edges, *x)).sum::<usize>();
                count.set(n);
                paths
                    + match curr {
                        "end" => 1,
                        _ => 0,
                    }
            }
        }
    }
    Ok(dfs(&parse_input(input), "start"))
}

fn part2(input: &str) -> Result<usize> {
    fn dfs(edges: &Edges, curr: &str, extra: usize) -> usize {
        let (nodes, count) = &edges[curr];
        let count = Rc::clone(count);
        match (curr, count.get(), extra) {
            ("start" | "end", 0, _) => 0,
            (_, 0, 0) => 0,
            (_, n, mut extra) => {
                match n {
                    0 => extra -= 1,
                    _ => count.set(n - 1),
                }
                let paths = nodes.iter().map(|x| dfs(edges, *x, extra)).sum::<usize>();
                if n == 0 {
                    count.set(n)
                }
                if n != 0 {
                    count.set(n)
                }
                paths
                    + match curr {
                        "end" => 1,
                        _ => 0,
                    }
            }
        }
    }
    Ok(dfs(&parse_input(input), "start", 1))
}

solution!(part1 => 3421, part2 => 84870);
