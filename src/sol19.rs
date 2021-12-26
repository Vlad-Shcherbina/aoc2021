type Vec3 = (i32, i32, i32);

pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut it = input.split_terminator('\n');
    let mut scanners = vec![];
    'outer: loop {
        let header = it.next().unwrap();
        assert!(header.starts_with("--- scanner "));

        scanners.push(vec![]);
        let scanner = scanners.last_mut().unwrap();
        loop {
            let Some(line) = it.next() else { break 'outer };
            if line.is_empty() { break }
            let (x, xy) = line.split_once(',').unwrap();
            let (y, z) = xy.split_once(',').unwrap();
            let x: i32 = x.parse().unwrap();
            let y: i32 = y.parse().unwrap();
            let z: i32 = z.parse().unwrap();
            scanner.push((x, y, z));
        }
    }

    let transformed_scanners: Vec<Vec<Vec<Vec3>>> =
        scanners.iter().map(|scanner| {
            (0..24).map(|tr| {
                scanner.iter().map(|&pt| transform(pt, tr)).collect()
            }).collect()
        }).collect();

    let mut all_pts = vec![];

    let mut scanner_positions = vec![(0, 0, 0); scanners.len()];
    let mut visited = vec![false; scanners.len()];
    visited[0] = true;
    let mut q = vec![scanners[0].clone()];
    while let Some(pts) = q.pop() {
        all_pts.extend_from_slice(&pts);
        for (i, tss) in transformed_scanners.iter().enumerate() {
            if visited[i] { continue }
            for ts in tss {
                let ds = possible_offsets(&pts, ts);
                match ds.as_slice() {
                    [] => {}
                    &[d] => {
                        visited[i] = true;
                        scanner_positions[i] = d;
                        let pts1: Vec<Vec3> = ts.iter().map(|&p| sub(p, d)).collect();
                        q.push(pts1);
                        break;
                    }
                    _ => panic!()
                }
            }
        }
    }
    assert!(visited.iter().all(|&v| v));

    all_pts.sort_unstable();
    all_pts.dedup();
    out(all_pts.len().to_string());

    let mut part2 = 0;
    for &s1 in &scanner_positions {
        for &s2 in &scanner_positions {
            let d = sub(s1, s2);
            part2 = part2.max(d.0.abs() + d.1.abs() + d.2.abs());
        }
    }
    out(part2.to_string());
}

fn transform(mut a: Vec3, mut k: u8) -> Vec3 {
    assert!(k < 24);
    if k % 2 == 1 {
        let t = a.0;
        a.0 = -a.1;
        a.1 = t;
    }
    k /= 2;
    for _ in 0 .. k % 3 {
        let t = a.0;
        a.0 = a.1;
        a.1 = a.2;
        a.2 = t;
    }
    k /= 3;
    match k {
        0 => {}
        1 => {
            a.0 *= -1;
            a.1 *= -1;
        }
        2 => {
            a.1 *= -1;
            a.2 *= -1;
        }
        3 => {
            a.2 *= -1;
            a.0 *= -1;
        }
        _ => panic!(),
    }
    a
}

fn possible_offsets(scanner1: &[Vec3], scanner2: &[Vec3]) -> Vec<Vec3> {
    let mut ds = Vec::with_capacity(scanner1.len() * scanner2.len());
    for &v1 in scanner1 {
        for &v2 in scanner2 {
            ds.push(sub(v2, v1));
        }
    }
    ds.sort_unstable();
    let mut bd = ds[0];
    let mut cnt = 1;
    let mut result = vec![];
    for &d in &ds[1..] {
        if bd == d {
            cnt += 1;
        } else {
            if cnt >= 12 {
                result.push(bd);
            }
            bd = d;
            cnt = 1;
        }
    }
    if cnt >= 12 {
        result.push(bd);
    }
    result
}

fn sub(a: Vec3, b: Vec3) -> Vec3 {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}
