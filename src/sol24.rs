use std::fmt::Write;
use std::ops::Range;

pub(crate) fn solve(mut input: &str, out: &mut dyn FnMut(String)) {
    let mut blocks = vec![];
    while !input.is_empty() {
        let (b, rest) = Block::parse(input);
        blocks.push(b);
        input = rest;
    }

    let mut allowed_ranges = vec![];
    let mut z = Range { start: 0, end: 1 };
    for b in blocks.iter().rev() {
        z = b.run_range_backwards(z);
        allowed_ranges.push(z.clone());
    }
    allowed_ranges.reverse();

    let mut digits: Vec<i64> = (1..=9).collect();
    for _ in 0..2 {
        digits.reverse();
        let mut ws = vec![];
        let res = rec(&digits, &blocks, &allowed_ranges, &mut ws, 0);
        assert!(res);

        let mut s = String::new();
        for w in ws {
            write!(s, "{}", w).unwrap();
        }
        out(s);
    }

}

fn rec(
    digits: &[i64],
    blocks: &[Block],
    allowed_ranges: &[Range<i64>],
    ws: &mut Vec<i64>,
    z: i64,
) -> bool {
    match blocks.split_first() {
        None => z == 0,
        Some((block, blocks)) => {
            let (rz, allowed_ranges) = allowed_ranges.split_first().unwrap();
            if !rz.contains(&z) {
                return false;
            }
            for &w in digits {
                let z1 = block.run(z, w);
                ws.push(w);
                if rec(digits, blocks, allowed_ranges, ws, z1) {
                    return true;
                }
                ws.pop().unwrap();
            }
            false
        }
    }
}

#[derive(Debug)]
struct Block {
    dz: i64,
    ax: i64,
    ay: i64,
}

impl Block {
    fn run_range_backwards(&self, z: Range<i64>) -> Range<i64> {
        let start = (z.start - self.ay).max(0);
        let z1 = Range { start, end: (z.end - self.ay).max(start) };
        let z1 = backwards_range_mul(z1, 26);
        let z1 = Range {
            start: z1.start.min(z.start),
            end: z1.end.max(z.end),
        };
        assert!(0 <= z1.start);
        assert!(z1.start <= z1.end);

        match self.dz {
            1 => z1,
            26 => backwards_range_div(z1, 26),
            _ => panic!()
        }
    }

    fn run(&self, mut z: i64, w: i64) -> i64 {
        let x = z % 26 + self.ax;
        match self.dz {
            1 => {}
            26 => z /= 26,
            _ => panic!()
        }
        if x != w {
            z *= 26;
            z += w + self.ay;
        }
        z
    }

    fn parse(input: &str) -> (Self, &str) {
        let input = input.strip_prefix(
"inp w
mul x 0
add x z
mod x 26
div z ").unwrap();
        let (dz, input) = input.split_once('\n').unwrap();
        let dz: i64 = dz.parse().unwrap();
        let input = input.strip_prefix("add x ").unwrap();
        let (ax, input) = input.split_once('\n').unwrap();
        let ax: i64 = ax.parse().unwrap();
        let input = input.strip_prefix(
"eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y ").unwrap();
        let (ay, input) = input.split_once('\n').unwrap();
        let ay: i64 = ay.parse().unwrap();
        assert!(ay >= 0);
        let input = input.strip_prefix("mul y x\nadd z y\n").unwrap();
        (Block { dz, ax, ay}, input)
    }
}

/*
mul x 0
add x z
mod x 26
div z <dz>
add x <ax>
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y <ay>
mul y x
add z y
*/

fn backwards_range_div(r: Range<i64>, d: i64) -> Range<i64> {
    assert!(r.start >= 0);
    Range { start: r.start * d, end: r.end * d }
}

#[test]
fn test_backwards_range_div() {
    for x1 in 0..10 {
        for x2 in x1..10 {
            let r = Range { start: x1, end: x2 };
            for d in 1..3 {
                let pr = backwards_range_div(r.clone(), d);
                if pr.start > 0 {
                    assert!(!r.contains(&((pr.start - 1) / d)));
                }
                assert!(!r.contains(&(pr.end / d)));
                for px in pr {
                    assert!(r.contains(&(px / d)));
                }
            }
        }
    }
}

fn backwards_range_mul(r: Range<i64>, m: i64) -> Range<i64> {
    assert!(r.start >= 0);
    Range {
        start: (r.start + m - 1) / m,
        end: (r.end + m - 1) / m,
    }
}

#[test]
fn test_backwards_range_mul() {
    for x1 in 0..10 {
        for x2 in x1..10 {
            let r = Range { start: x1, end: x2 };
            for m in 1..3 {
                let pr = backwards_range_mul(r.clone(), m);
                if pr.start > 0 {
                    assert!(!r.contains(&((pr.start - 1) * m)));
                }
                assert!(!r.contains(&(pr.end * m)));
                for px in pr {
                    assert!(r.contains(&(px * m)));
                }
            }
        }
    }
}
