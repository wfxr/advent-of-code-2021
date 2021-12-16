use crate::{solution, Result};
use std::iter;

enum Packet {
    Literal { ver: u32, _val: u64 },
    Operator { ver: u32, _tag: u32, subs: Vec<Packet> },
}

fn parse_input(s: &str) -> Option<Packet> {
    fn next_int<const N: u32>(iter: &mut impl Iterator<Item = u32>) -> Option<u32> {
        let mut ret = 0;
        for _ in 0..N {
            ret = 2 * ret + iter.next()?;
        }
        Some(ret)
    }
    fn parse_packet(iter: &mut impl Iterator<Item = u32>) -> Option<Packet> {
        let ver = next_int::<3>(iter)?;
        let tag = next_int::<3>(iter)?;
        match tag {
            4 => {
                let mut val = 0;
                loop {
                    let num = iter.next()?;
                    val = 16 * val + next_int::<4>(iter)? as u64;
                    if num == 0 {
                        break;
                    }
                }
                Some(Packet::Literal { ver, _val: val })
            }
            _ => {
                let subs: Vec<_> = match iter.next()? {
                    0 => {
                        let n = next_int::<15>(iter)? as usize;
                        let mut iter = iter.take(n).collect::<Vec<_>>().into_iter();
                        iter::from_fn(|| parse_packet(&mut iter)).collect()
                    }
                    _ => {
                        let n = next_int::<11>(iter)?;
                        (0..n).map(|_| parse_packet(iter)).collect::<Option<_>>()?
                    }
                };
                Some(Packet::Operator { ver, _tag: tag, subs })
            }
        }
    }
    let iter = &mut s
        .chars()
        .filter_map(|c| c.to_digit(16))
        .flat_map(|x| (0..4).rev().map(move |n| (x >> n) & 1));
    parse_packet(iter)
}

fn sum_versions(packet: &Packet) -> u32 {
    match packet {
        Packet::Literal { ver, .. } => *ver,
        Packet::Operator { ver, subs, .. } => *ver + subs.iter().map(sum_versions).sum::<u32>(),
    }
}

fn part1(input: &str) -> Result<u32> {
    parse_input(input)
        .ok_or_else(|| "can not decode".into())
        .map(|packet| sum_versions(&packet))
}

fn part2(_input: &str) -> Result<usize> {
    todo!()
}

solution!(part1 => 871, part2 => todo!());

#[cfg(test)]
mod example {
    crate::test!(part1,
        t1: "8A004A801A8002F478" => 16,
        t2: "620080001611562C8802118E34" => 12,
        t3: "C0015000016115A2E0802F182340" => 23,
        t4: "A0016C880162017C3686B18A3D4780" => 31,
    );
}
