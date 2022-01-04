use fxhash::FxHashSet as HashSet;

pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let input: Vec<char> = input.chars().filter(|c| ('A'..='D').contains(c)).collect();
    assert_eq!(input.len(), 8);

    let mut initial_state = State {
        hallway: [None; 11],
        rooms: vec![[None; 4]; 2],
    };
    for (i, row) in initial_state.rooms.iter_mut().enumerate() {
        for (j, c) in row.iter_mut().enumerate() {
            *c = Some(input[i * 4 + j] as u8 - b'A');
        }
    }

    let mut visited = HashSet::default();
    let mut frontier = std::collections::BinaryHeap::new();
    frontier.push(HeapEntry {
        cost: 0,
        state: initial_state.clone(),
    });
    loop {
        let HeapEntry { cost, state: s } = frontier.pop().unwrap();
        if s.is_final() {
            out(cost.to_string());
            break;
        }
        if !visited.insert(s.clone()) {
            continue;
        }
        for (d, s2) in s.adj() {
            if visited.contains(&s2) { continue }
            let cost2 = cost + d;
            frontier.push(HeapEntry {
                cost: cost2,
                state: s2,
            });
        }
    }
}

#[derive(Eq)]
struct HeapEntry {
    cost: i32,
    state: State,
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialEq for HeapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const ROOM_POS: [usize; 4] = [2, 4, 6, 8];
const MOVE_COST: [i32; 4] = [1, 10, 100, 1000];

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    hallway: [Option<u8>; 11],
    rooms: Vec<[Option<u8>; 4]>,
}

impl State {
    fn is_final(&self) -> bool {
        self.rooms.iter()
            .all(|layer| layer == &[Some(0), Some(1), Some(2), Some(3)])
    }

    fn adj(&self) -> Vec<(i32, State)> {
        let mut res = vec![];
        for i in 0..4 {
            for j in 0..self.rooms.len() {
                let Some(pod) = self.rooms[j][i] else { continue };
                for dx in [-1, 1] {
                    let mut x = ROOM_POS[i] as i32;
                    let mut dist = j + 1;
                    x += dx;
                    dist += 1;
                    while 0 <= x && x < self.hallway.len() as i32 {
                        if self.hallway[x as usize].is_some() {
                            break;
                        }
                        if !ROOM_POS.contains(&(x as usize)) {
                            let mut s2 = self.clone();
                            s2.rooms[j][i] = None;
                            s2.hallway[x as usize] = Some(pod);
                            res.push((dist as i32 * MOVE_COST[pod as usize], s2));
                        }
                        x += dx;
                        dist += 1;
                    }
                }
                break;
            }
        }

        for (x, &pod) in self.hallway.iter().enumerate() {
            let Some(pod) = pod else { continue };
            let i = pod as usize;

            let can_enter = self.rooms.iter()
                .all(|layer| layer[i].map_or(true, |p| p == pod));
            if !can_enter {
                continue;
            }

            if self.rooms[0][i].is_some() {
                continue;
            }

            let x2 = ROOM_POS[pod as usize];
            assert_ne!(x, x2);
            let dist = if x < x2 {
                if self.hallway[x + 1 ..= x2].iter().any(Option::is_some) {
                    continue;
                }
                x2 - x
            } else {
                if self.hallway[x2 .. x].iter().any(Option::is_some) {
                    continue;
                }
                x - x2
            };

            for j in 0..self.rooms.len() {
                if self.rooms[j][i].is_some() {
                    break;
                }
                let mut s2 = self.clone();
                s2.hallway[x] = None;
                s2.rooms[j][i] = Some(pod);
                let cost = (dist + j + 1) as i32 * MOVE_COST[i];
                res.push((cost, s2));
            }
        }
        res
    }
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn to_char(x: Option<u8>) -> char {
            match x {
                None => '.',
                Some(x) => (x + b'A') as char,
            }
        }
        writeln!(f, "State {{")?;
        write!(f, "  ")?;
        for x in self.hallway {
            write!(f, "{}", to_char(x))?;
        }
        writeln!(f)?;
        for row in &self.rooms {
            write!(f, "   ")?;
            for &pod in row {
                write!(f, " {}", to_char(pod))?;
            }
            writeln!(f)?;
        }
        write!(f, "}}")
    }
}
