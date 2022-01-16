use fxhash::FxHashSet as HashSet;

pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let input: Vec<char> = input.chars().filter(|c| ('A'..='D').contains(c)).collect();
    assert_eq!(input.len(), 8);

    let mut initial_state = State {
        hallway: [None; 11],
        rooms: [[None; 4]; 2],
    };
    for (i, row) in initial_state.rooms.iter_mut().enumerate() {
        for (j, c) in row.iter_mut().enumerate() {
            *c = Some(input[i * 4 + j] as u8 - b'A');
        }
    }

    fn solve<const N: usize>(initial_state: &State<N>) -> i32 {
        // TODO: this implementation of A* (basically, minor tweak to Dijkstra)
        // is incorrect if heuristic function is not consistent (aka monotone).
        let mut visited = HashSet::default();
        let mut frontier = std::collections::BinaryHeap::new();
        frontier.push(HeapEntry {
            heuristic_cost: initial_state.heuristic_dist_to_final(),
            actual_cost: 0,
            state: initial_state.clone(),
        });
        loop {
            let HeapEntry {
                heuristic_cost: _,
                actual_cost,
                state: s,
            } = frontier.pop().unwrap();
            if s.is_final() {
                log::info!("{} visited", visited.len());
                log::info!("{} frontier", frontier.len());
                log::info!("max frontier entry: {}", frontier.iter().map(|e| e.heuristic_cost).max().unwrap());
                return actual_cost;
            }
            if !visited.insert(s.clone()) {
                continue;
            }
            for (d, s2) in s.adj() {
                if visited.contains(&s2) { continue }
                frontier.push(HeapEntry {
                    heuristic_cost: actual_cost + d + s2.heuristic_dist_to_final(),
                    actual_cost: actual_cost + d,
                    state: s2,
                });
            }
        }
    }

    out(solve(&initial_state).to_string());
    out(solve(&initial_state.unfold()).to_string());
}

#[derive(Eq)]
struct HeapEntry<const N: usize> {
    heuristic_cost: i32,
    actual_cost: i32,
    state: State<N>,
}

impl<const N: usize> Ord for HeapEntry<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heuristic_cost.cmp(&other.heuristic_cost).reverse()
    }
}

impl<const N: usize> PartialEq for HeapEntry<N> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl<const N: usize> PartialOrd for HeapEntry<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const ROOM_POS: [usize; 4] = [2, 4, 6, 8];
const MOVE_COST: [i32; 4] = [1, 10, 100, 1000];

#[derive(Clone, PartialEq, Eq, Hash)]
struct State<const N: usize> {
    hallway: [Option<u8>; 11],
    rooms: [[Option<u8>; 4]; N],
}

impl State<2> {
    fn unfold(&self) -> State<4> {
        State {
            hallway: self.hallway,
            rooms: [
                self.rooms[0],
                [Some(3), Some(2), Some(1), Some(0)],  // D C B A
                [Some(3), Some(1), Some(0), Some(2)],  // D B A C
                self.rooms[1],
            ]
        }
    }
}

impl<const N: usize> State<N> {
    fn is_final(&self) -> bool {
        self.rooms.iter()
            .all(|layer| layer == &[Some(0), Some(1), Some(2), Some(3)])
    }

    fn heuristic_dist_to_final(&self) -> i32 {
        let mut res = 0;
        for (i, cell) in self.hallway.iter().enumerate() {
            if let &Some(q) = cell {
                res += ((ROOM_POS[q as usize] as i32 - i as i32).abs() + 1)
                    * MOVE_COST[q as usize];
            }
        }
        for (i, layer) in self.rooms.iter().enumerate() {
            for (j, cell) in layer.iter().enumerate() {
                if let &Some(q) = cell {
                    if q as usize != j {
                        res += (i as i32 + 1 + (ROOM_POS[j] as i32 - ROOM_POS[q as usize] as i32).abs() + 1)
                            * MOVE_COST[q as usize]
                    }
                }
            }
        }
        res
    }

    fn adj(&self) -> Vec<(i32, State<N>)> {
        let mut res = vec![];
        for (i, &room_pos) in ROOM_POS.iter().enumerate() {
            for j in 0..self.rooms.len() {
                let Some(pod) = self.rooms[j][i] else { continue };
                for dx in [-1, 1] {
                    let mut x = room_pos as i32;
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

impl<const N: usize> std::fmt::Debug for State<N> {
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
