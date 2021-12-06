use std::collections::HashMap;

struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let lines: Vec<Line> = input.split_terminator('\n')
    .map(|line| {
        let (left, right) = line.split_once(" -> ").unwrap();
        let (x, y) = left.split_once(',').unwrap();
        let x1 = x.parse().unwrap();
        let y1 = y.parse().unwrap();
        let (x, y) = right.split_once(',').unwrap();
        let x2 = x.parse().unwrap();
        let y2 = y.parse().unwrap();
        Line { x1, y1, x2, y2 }
    }).collect();

    let mut counts: HashMap<(i32, i32), i32> = HashMap::new();
    for line in &lines {
        if line.x1 == line.x2 {
            let x = line.x1;
            let y1 = line.y1.min(line.y2);
            let y2 = line.y1.max(line.y2);
            for y in y1..=y2 {
                *counts.entry((x, y)).or_default() += 1;
            }
        } else if line.y1 == line.y2 {
            let y = line.y1;
            let x1 = line.x1.min(line.x2);
            let x2 = line.x1.max(line.x2);
            for x in x1..=x2 {
                *counts.entry((x, y)).or_default() += 1;
            }
        }
    }
    let res = counts.values().filter(|&&c| c > 1).count();
    out(res.to_string());
}
