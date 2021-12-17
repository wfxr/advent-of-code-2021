use crate::{solution, Result};

enum Packet {
    Literal { ver: u32, val: u64 },
    Operator { ver: u32, tag: u32, subs: Vec<Packet> },
}

fn parse_input(s: &str) -> Option<Packet> {
    fn read<const N: u32>(iter: &mut impl Iterator<Item = u32>) -> Option<u32> {
        let mut ret = 0;
        for _ in 0..N {
            ret = ret << 1 | iter.next()?;
        }
        Some(ret)
    }
    fn parse_packet(iter: &mut impl Iterator<Item = u32>) -> Option<Packet> {
        let ver = read::<3>(iter)?;
        let tag = read::<3>(iter)?;
        match tag {
            4 => {
                let mut val = 0;
                loop {
                    let num = iter.next()?;
                    val = (val << 4) + read::<4>(iter)? as u64;
                    if num == 0 {
                        break;
                    }
                }
                Some(Packet::Literal { ver, val })
            }
            _ => {
                let subs: Vec<_> = match iter.next()? {
                    0 => {
                        let n = read::<15>(iter)? as usize;
                        let mut iter = iter.take(n).collect::<Vec<_>>().into_iter();
                        std::iter::from_fn(|| parse_packet(&mut iter)).collect()
                    }
                    _ => {
                        let n = read::<11>(iter)?;
                        (0..n).map(|_| parse_packet(iter)).collect::<Option<_>>()?
                    }
                };
                Some(Packet::Operator { ver, tag, subs })
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

fn eval(packet: &Packet) -> Result<u64> {
    Ok(match packet {
        Packet::Literal { val, .. } => *val,
        Packet::Operator { tag, subs, .. } => match tag {
            0 => subs.iter().map(eval).sum::<Result<_>>()?,
            1 => subs.iter().map(eval).product::<Result<_>>()?,
            2 => subs.iter().try_fold(u64::MAX, |min, x| eval(x).map(|v| v.min(min)))?,
            3 => subs.iter().try_fold(u64::MIN, |min, x| eval(x).map(|v| v.max(min)))?,
            5 => (eval(subs.get(0).ok_or("no lhs")?)? > eval(subs.get(1).ok_or("no rhs")?)?) as u64,
            6 => (eval(subs.get(0).ok_or("no lhs")?)? < eval(subs.get(1).ok_or("no rhs")?)?) as u64,
            7 => (eval(subs.get(0).ok_or("no lhs")?)? == eval(subs.get(1).ok_or("no rhs")?)?) as u64,
            x => return Err(format!("unexpected type id: {}", x).into()),
        },
    })
}

fn part1(input: &str) -> Result<u32> {
    parse_input(input)
        .ok_or_else(|| "can not decode".into())
        .map(|packet| sum_versions(&packet))
}

fn part2(input: &str) -> Result<u64> {
    parse_input(input)
        .ok_or_else(|| "can not decode".into())
        .and_then(|packet| eval(&packet))
}

solution!(part1 => 871, part2 => 68703010504);

#[cfg(test)]
mod example {
    crate::test!(part1,
        t1: "8A004A801A8002F478"             => 16,
        t2: "620080001611562C8802118E34"     => 12,
        t3: "C0015000016115A2E0802F182340"   => 23,
        t4: "A0016C880162017C3686B18A3D4780" => 31,
    );

    crate::test!(part2,
        t1: "C200B40A82"                 => 3,
        t2: "04005AC33890"               => 54,
        t3: "880086C3E88112"             => 7,
        t4: "CE00C43D881120"             => 9,
        t5: "D8005AC2A8F0"               => 1,
        t6: "F600BC2D8F"                 => 0,
        t7: "9C005AC2F8F0"               => 0,
        t8: "9C0141080250320F1802104A08" => 1,
    );
}
