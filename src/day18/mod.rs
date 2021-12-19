use crate::{solution, Result};
use std::str::FromStr;
use Number::*;

enum Number {
    Literal(u8),
    Pair(Box<Number>, Box<Number>),
}

impl Number {
    fn reduce(mut self) -> Self {
        match self.explode(0).0 {
            true => self.reduce(),
            false => match self.split() {
                true => self.reduce(),
                false => self,
            },
        }
    }

    fn explode(&mut self, depth: usize) -> (bool, u8, u8) {
        match self {
            Literal(..) => (false, 0, 0),
            Pair(lhs, rhs) => match (lhs.as_ref(), rhs.as_ref(), depth) {
                (&Literal(l), &Literal(r), 4..) => {
                    *self = Literal(0);
                    (true, l, r)
                }
                _ => match lhs.explode(depth + 1) {
                    (true, left, right) => {
                        rhs.add_left(right);
                        (true, left, 0)
                    }
                    _ => match rhs.explode(depth + 1) {
                        (true, left, right) => {
                            lhs.add_right(left);
                            (true, 0, right)
                        }
                        _ => (false, 0, 0),
                    },
                },
            },
        }
    }

    fn split(&mut self) -> bool {
        match self {
            &mut Literal(x) => match x {
                10.. => {
                    let l = x / 2;
                    let r = x - l;
                    *self = Pair(box Literal(l), box Literal(r));
                    true
                }
                _ => false,
            },
            Pair(lhs, rhs) => lhs.split() || rhs.split(),
        }
    }

    fn add_left(&mut self, x: u8) {
        match self {
            _ if x == 0 => {}
            Literal(n) => *n += x,
            Pair(left, _) => left.add_left(x),
        }
    }

    fn add_right(&mut self, x: u8) {
        match self {
            _ if x == 0 => {}
            Literal(n) => *n += x,
            Pair(_, right) => right.add_right(x),
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Literal(x) => *x as u32,
            Pair(lhs, rhs) => lhs.magnitude() * 3 + rhs.magnitude() * 2,
        }
    }
}

impl FromStr for Number {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        fn parse(tokens: &mut dyn Iterator<Item = char>) -> Result<Number> {
            match tokens.next() {
                Some('[') => Ok(Pair(box parse(tokens)?, box parse(tokens)?)),
                Some(c) if c.is_digit(10) => Ok(Literal(c as u8 - b'0')),
                Some(c) => Err(format!("unexpected token: {}", c).into()),
                None => Err("missing token".into()),
            }
        }
        parse(&mut s.chars().filter(|&c| c != ',' && c != ']'))
    }
}

fn part1(input: &str) -> Result<u32> {
    Ok(input
        .lines()
        .map(|l| l.parse::<Number>())
        .try_fold(Literal(0), |acc, x| x.map(|x| Pair(box acc, box x).reduce()))?
        .magnitude())
}

fn part2(_input: &str) -> Result<usize> {
    todo!()
}

solution!(part1 => 4417, part2 => todo!());

#[cfg(test)]
mod example {
    crate::test!(part1, t1: "
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
".trim() => 4140);
}
