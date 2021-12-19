#[derive(Debug, Clone)]
enum Number {
    Regular(i32),
    Pair(Box<(Number, Number)>),
}
use Number::*;

pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let numbers: Vec<_> = input.split_terminator('\n')
        .map(Number::parse)
        .collect();
    let mut s = numbers[0].clone();
    for x in &numbers[1..] {
        s = Pair(Box::new((s, x.clone())));
        s.reduce();
    }
    out(s.magnitude().to_string());

    let mut max_mag = 0;
    for (i, x) in numbers.iter().enumerate() {
        for (j, y) in numbers.iter().enumerate() {
            if i != j {
                let mut z = Pair(Box::new((x.clone(), y.clone())));
                z.reduce();
                max_mag = max_mag.max(z.magnitude());
            }
        }
    }
    out(max_mag.to_string());
}

impl Number {
    fn magnitude(&self) -> i32 {
        match self {
            &Regular(x) => x,
            Pair(b) => 3 * b.0.magnitude() + 2 * b.1.magnitude()
        }
    }

    fn reduce(&mut self) {
        while self.explode() || self.split() {}
    }

    fn split(&mut self) -> bool {
        match self {
            &mut Regular(x) => if x >= 10 {
                *self = Pair(Box::new((Regular(x / 2), Regular((x + 1) / 2))));
                true
            } else {
                false
            }
            Pair(b) => b.0.split() || b.1.split()
        }
    }

    fn explode(&mut self) -> bool {
        self.explode_inner(0).is_some()
    }

    fn explode_inner(&mut self, depth: i32) -> Option<(Option<i32>, Option<i32>)> {
        match self {
            Regular(_) => None,
            Pair(b) => if depth == 4 {
                let &Regular(left) = &b.0 else { panic!() };
                let &Regular(right) = &b.1 else { panic!() };
                *self = Regular(0);
                Some((Some(left), Some(right)))
            } else if let Some((left, right)) = b.0.explode_inner(depth + 1) {
                if let Some(right) = right {
                    b.1.inc_left_leaf(right);
                }
                Some((left, None))
            } else if let Some((left, right)) = b.1.explode_inner(depth + 1) {
                if let Some(left) = left {
                    b.0.inc_right_leaf(left);
                }
                Some((None, right))
            } else {
                None
            }
        }
    }

    fn inc_left_leaf(&mut self, delta: i32) {
        let mut q = self;
        loop {
            match q {
                Regular(x) => {
                    *x += delta;
                    break;
                }
                Pair(b) => q = &mut b.0,
            }
        }
    }

    fn inc_right_leaf(&mut self, delta: i32) {
        let mut q = self;
        loop {
            match q {
                Regular(x) => {
                    *x += delta;
                    break;
                }
                Pair(b) => q = &mut b.1,
            }
        }
    }

    fn parse(s: &str) -> Number {
        let mut bytes = s.as_bytes();
        let res = Number::read(&mut bytes);
        assert!(bytes.is_empty());
        res
    }

    fn read(bytes: &mut &[u8]) -> Number {
        if bytes[0] == b'[' {
            *bytes = &bytes[1..];
            let left = Number::read(bytes);
            assert_eq!(bytes[0], b',');
            *bytes = &bytes[1..];
            let right = Number::read(bytes);
            assert_eq!(bytes[0], b']');
            *bytes = &bytes[1..];
            Pair(Box::new((left, right)))
        } else {
            let mut res = 0;
            while !bytes.is_empty() && (b'0'..=b'9').contains(&bytes[0]) {
                res *= 10;
                res += (bytes[0] - b'0') as i32;
                *bytes = &bytes[1..];
            }
            Regular(res)
        }
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Regular(x) => write!(f, "{}", x),
            Pair(b) => write!(f, "[{},{}]", b.0, b.1),
        }
    }
}
