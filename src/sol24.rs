use std::fmt::Write;

pub(crate) fn solve(mut input: &str, out: &mut dyn FnMut(String)) {
    let mut blocks = vec![];
    while !input.is_empty() {
        let (b, rest) = Block::parse(input);
        blocks.push(b);
        input = rest;
    }

    let mut ws = vec![];
    let res = rec(&blocks, &mut ws, 0);
    assert!(res);

    let mut s = String::new();
    for w in ws {
        write!(s, "{}", w).unwrap();
    }
    out(s);
}

fn rec(blocks: &[Block], ws: &mut Vec<i64>, z: i64) -> bool {
    match blocks.split_first() {
        None => {
            return z == 0;
        }
        Some((block, blocks)) => {
            let mut tz = z;
            for b in blocks {
                match b.dz {
                    1 => {}
                    26 => tz /= 26,
                    _ => panic!(),
                }
            }
            if tz.abs() > 30 {
                return false;
            }
            for w in (1 .. 9 + 1).rev() {
                let z1 = block.run(z, w);
                ws.push(w);
                if rec(blocks, ws, z1) {
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
