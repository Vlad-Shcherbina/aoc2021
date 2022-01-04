use fxhash::FxHashSet as HashSet;
use fxhash::FxHashMap as HashMap;

pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let input: Vec<char> = input.chars().filter(|c| ('A'..='D').contains(c)).collect();
    assert_eq!(input.len(), 8);

    let mut initial_state = State {
        hallway: [None; 11],
        rooms: [[None; 2]; 4],
    };
    for (i, room) in initial_state.rooms.iter_mut().enumerate() {
        for (j, c) in room.iter_mut().enumerate() {
            *c = Some(input[i + 4 * j] as u8 - b'A');
        }
    }

    let mut visited = HashSet::default();
    let mut frontier = HashMap::default();
    frontier.insert(initial_state, 0);
    loop {
        let (s, _) = frontier.iter().min_by_key(|&(_s, &cost)| cost).unwrap();
        let s = s.clone();
        let (s, cost) = frontier.remove_entry(&s).unwrap();
        if s.is_final() {
            out(cost.to_string());
            break;
        }
        let was_new = visited.insert(s.clone());
        for (d, s2) in s.adj() {
            if visited.contains(&s2) { continue }
            let cost2 = cost + d;
            frontier.entry(s2)
                .and_modify(|c2| *c2 = (*c2).min(cost2))
                .or_insert(cost2);
        }
        assert!(was_new);
    }
}

const ROOM_POS: [usize; 4] = [2, 4, 6, 8];
const MOVE_COST: [i32; 4] = [1, 10, 100, 1000];

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    hallway: [Option<u8>; 11],
    rooms: [[Option<u8>; 2]; 4],
}

impl State {
    fn is_final(&self) -> bool {
        self.rooms.iter().enumerate().all(|(i, room)| {
            room[0] == Some(i as u8) &&
            room[1] == Some(i as u8)
        })
    }

    fn adj(&self) -> Vec<(i32, State)> {
        let mut res = vec![];
        for (i, room) in self.rooms.iter().enumerate() {
            for (j, &pod) in room.iter().enumerate() {
                let Some(pod) = pod else { continue };
                if j == 1 && room[0].is_some() { continue }

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
                            s2.rooms[i][j] = None;
                            s2.hallway[x as usize] = Some(pod);
                            res.push((dist as i32 * MOVE_COST[pod as usize], s2));
                        }
                        x += dx;
                        dist += 1;
                    }
                }
            }
        }

        for (x, &pod) in self.hallway.iter().enumerate() {
            let Some(pod) = pod else { continue };
            let room = &self.rooms[pod as usize];
            if room[0].is_some() { continue };
            if let Some(q) = room[1] {
                if q != pod { continue }
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

            #[allow(clippy::needless_range_loop)]
            for j in 0..2 {
                if room[j].is_some() { break; }
                let mut s2 = self.clone();
                s2.hallway[x] = None;
                s2.rooms[pod as usize][j] = Some(pod);
                let cost = (dist + j + 1) as i32 * MOVE_COST[pod as usize];
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
        for row in 0..2 {
            write!(f, "   ")?;
            for room in &self.rooms {
                write!(f, " {}", to_char(room[row]))?;
            }
            writeln!(f)?;
        }
        write!(f, "}}")
    }
}
