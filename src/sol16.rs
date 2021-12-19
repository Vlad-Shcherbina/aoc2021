enum Packet {
    Literal {
        version: i32,
        value: i64,
    },
    Operator {
        version: i32,
        tp: i32,
        children: Vec<Packet>,
    },
}
use Packet::*;

pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let input = input.trim_end();
    let mut bits = vec![0u8; input.len() * 4];
    for (i, c) in input.bytes().enumerate() {
        let c = if (b'0'..=b'9').contains(&c) {
            c - b'0'
        } else if (b'A'..=b'F').contains(&c) {
            c - b'A' + 10
        } else { panic!() };
        assert!(c < 16);
        bits[i * 4] = c / 8;
        bits[i * 4 + 1] = c / 4 % 2;
        bits[i * 4 + 2] = c / 2 % 2;
        bits[i * 4 + 3] = c % 2;
    }
    let mut bits: &[u8] = &bits;
    let root = read_packet(&mut bits);
    for &bit in bits {
        assert_eq!(bit, 0);
    }

    out(part1(&root).to_string());
    out(part2(&root).to_string());
}

fn part1(p: &Packet) -> i32 {
    match p {
        &Literal { version, .. } => version,
        Operator { version, children, ..} =>
            *version + children.iter().map(part1).sum::<i32>()
    }
}

fn part2(p: &Packet) -> i64 {
    match p {
        &Literal { value, .. } => value,
        Operator { tp, children, .. } => match tp {
            0 => children.iter().map(part2).sum(),
            1 => {
                let mut res = 1;
                for child in children {
                    res *= part2(child);
                }
                res
            }
            2 => children.iter().map(part2).min().unwrap(),
            3 => children.iter().map(part2).max().unwrap(),
            4 => unreachable!("literal"),
            5 => {
                assert_eq!(children.len(), 2);
                (part2(&children[0]) > part2(&children[1])) as i64
            }
            6 => {
                assert_eq!(children.len(), 2);
                (part2(&children[0]) < part2(&children[1])) as i64
            }
            7 => {
                assert_eq!(children.len(), 2);
                (part2(&children[0]) == part2(&children[1])) as i64
            }
            _ => panic!(),
        }
    }
}

fn read_packet(bits: &mut &[u8]) -> Packet {
    let version = read_num(3, bits);
    let tp = read_num(3, bits);
    if tp == 4 { // literal value
        let mut value = 0i64;
        loop {
            let group = read_num(5, bits);
            value *= 16;
            value += group as i64 % 16;
            if group < 16 {
                return Literal { version, value };
            }
        }
    } else {
        let indicator = read_num(1, bits);
        if indicator == 0 {
            let len = read_num(15, bits) as usize;
            let target_remaining_len = bits.len() - len;
            let mut children = vec![];
            while bits.len() > target_remaining_len {
                children.push(read_packet(bits));
            }
            assert_eq!(bits.len(), target_remaining_len);
            Operator { version, tp, children }
        } else {
            let num = read_num(11, bits);
            let mut children = Vec::with_capacity(num as usize);
            for _ in 0..num {
                children.push(read_packet(bits));
            }
            Operator { version, tp, children }
        }
    }
}

fn read_num(len: i32, bits: &mut &[u8]) -> i32 {
    let mut res = 0;
    for _ in 0..len {
        res *= 2;
        res += bits[0] as i32;
        *bits = &bits[1..];
    }
    res
}
