use std::{cell::RefCell, iter::once, rc::Rc};

use crate::{solution, Result};

fn parse_input(input: &str) -> Result<(Vec<u8>, Vec<[u8; 3]>)> {
    let (template, rules) = input.split_once("\n\n").ok_or("missing rules")?;
    let template = template.as_bytes().to_vec();
    let rules = rules
        .lines()
        .map(|l| {
            l.bytes()
                .filter(|c| c.is_ascii_alphabetic())
                .collect::<Vec<_>>()
                .try_into()
                .map_err(|_| "invalid rule".into())
        })
        .collect::<Result<_>>()?;
    Ok((template, rules))
}

fn part1(input: &str) -> Result<u32> {
    let (mut template, rules) = parse_input(input)?;
    for _ in 0..10 {
        let fold_template: Vec<(u8, Rc<RefCell<Vec<u8>>>)> = template.iter().map(|&c| (c, Rc::default())).collect();
        rules.iter().for_each(|&[a, b, c]| {
            fold_template.array_windows().for_each(|[(l, ll), (r, _)]| {
                if *l == a && *r == b {
                    ll.borrow_mut().push(c)
                }
            });
        });
        template = fold_template
            .into_iter()
            .flat_map(|(c, cc)| once(c).chain(cc.borrow().iter().copied()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
    }

    let counter = template.into_iter().fold([0u32; 26], |mut acc, c| {
        acc[(c - b'A') as usize] += 1;
        acc
    });

    Ok(match (counter.iter().max(), counter.iter().filter(|&&x| x > 0).min()) {
        (Some(max), Some(min)) => max - min,
        _ => 0,
    })
}

fn part2(_input: &str) -> Result<usize> {
    Ok(0)
}

solution!(part1 => 3247, part2 => todo!());
