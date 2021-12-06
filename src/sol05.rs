struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut lines: Vec<Line> = input.split_terminator('\n')
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

    let split = lines.iter_mut().partition_in_place(
        |line| line.x1 == line.x2 || line.y1 == line.y2);

    const SIZE: usize = 1000;
    let mut counts = vec![0i16; SIZE * SIZE];
    for lines in [&lines[..split], &lines[split..]] {
        for line in lines {
            let mut x = line.x1;
            let mut y = line.y1;
            let dx = (line.x2 - line.x1).signum();
            let dy = (line.y2 - line.y1).signum();
            loop {
                assert!(0 <= x && x < SIZE as i32);
                assert!(0 <= y && y < SIZE as i32);
                counts[x as usize + y as usize * SIZE] += 1;
                if x == line.x2 && y == line.y2 {
                    break;
                }
                x += dx;
                y += dy;
            }
        }
        out(counts.iter().filter(|&&c| c > 1).count().to_string());
    }
}
