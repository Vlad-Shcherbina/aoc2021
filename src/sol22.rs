pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut cubes = vec![];
    for line in input.split_terminator('\n') {
        let (is_on, line) = line.split_once(' ').unwrap();
        let is_on = match is_on {
            "on" => true,
            "off" => false,
            _ => panic!(),
        };

        let cube = Cube::parse(line);
        cubes.push((is_on, cube));
    }

    for d in [50, 1_000_000] {
        let r = (-d, d + 1);
        out(rec(&cubes, Cube { x: r, y: r, z: r }).to_string());
    }
}

fn rec(cubes: &[(bool, Cube)], vol: Cube) -> i64 {
    let Some((&(is_on, cube), cubes)) = cubes.split_last() else { return 0; };
    let (inside, outside) = vol.subdivide(&cube);
    let mut res = 0;
    if is_on {
        if let Some(inside) = inside {
            res += inside.volume();
        }
    }
    for out in outside {
        res += rec(cubes, out);
    }
    res
}

#[derive(Clone, Copy, Debug)]
struct Cube {
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

impl Cube {
    fn parse(s: &str) -> Cube {
        let s = s.strip_prefix("x=").unwrap();
        let (x, s) = s.split_once(",y=").unwrap();
        let (y, z) = s.split_once(",z=").unwrap();
        Cube {
            x: parse_range(x),
            y: parse_range(y),
            z: parse_range(z),
        }
    }

    fn intersect(&self, other: &Cube) -> Cube {
        Cube {
            x: intersect_ranges(self.x, other.x),
            y: intersect_ranges(self.y, other.y),
            z: intersect_ranges(self.z, other.z),
        }
    }

    fn is_empty(&self) -> bool {
        self.x.0 >= self.x.1 ||
        self.y.0 >= self.y.1 ||
        self.z.0 >= self.z.1
    }

    fn volume(&self) -> i64 {
        (self.x.1 - self.x.0) as i64 *
        (self.y.1 - self.y.0) as i64 *
        (self.z.1 - self.z.0) as i64
    }

    fn subdivide(&self, other: &Cube) -> (Option<Cube>, Vec<Cube>) {
        let intr = self.intersect(other);
        if intr.is_empty() {
            return (None, vec![*self]);
        }
        let mut outside = vec![];
        if self.x.0 < other.x.0 {
            outside.push(Cube {
                x: (self.x.0, other.x.0),
                y: self.y,
                z: self.z,
            });
        }
        if other.x.1 < self.x.1 {
            outside.push(Cube {
                x: (other.x.1, self.x.1),
                y: self.y,
                z: self.z,
            });
        }
        if self.y.0 < other.y.0 {
            outside.push(Cube {
                x: intr.x,
                y: (self.y.0, other.y.0),
                z: self.z,
            });
        }
        if other.y.1 < self.y.1 {
            outside.push(Cube {
                x: intr.x,
                y: (other.y.1, self.y.1),
                z: self.z,
            });
        }
        if self.z.0 < other.z.0 {
            outside.push(Cube {
                x: intr.x,
                y: intr.y,
                z: (self.z.0, other.z.0),
            });
        }
        if other.z.1 < self.z.1 {
            outside.push(Cube {
                x: intr.x,
                y: intr.y,
                z: (other.z.1, self.z.1),
            })
        }

        assert_eq!(
            self.volume(),
            intr.volume() + outside.iter().map(Cube::volume).sum::<i64>(),
            "subdivide({:?}, {:?}) -> ({:?}, {:?})", self, other, intr, outside,
        );
        (Some(intr), outside)
    }
}

fn intersect_ranges(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0.max(b.0), a.1.min(b.1))
}

fn parse_range(s: &str) -> (i32, i32) {
    let (min, max) = s.split_once("..").unwrap();
    (min.parse().unwrap(), max.parse::<i32>().unwrap() + 1)
}
