use fxhash::FxHashSet as HashSet;

pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut it = input.split_terminator('\n');
    let rule = it.next().unwrap();
    let rule: Vec<bool> = rule.bytes().map(to_bool).collect();

    let empty = it.next().unwrap();
    assert!(empty.is_empty());

    let mut default = false;
    let mut points = HashSet::default();
    for (i, line) in it.enumerate() {
        for (j, c) in line.bytes().enumerate() {
            let v = to_bool(c);
            if v != default {
                points.insert((i as i32, j as i32));
            }
        }
    }

    for _ in 0..2 {
        (default, points) = step(&rule, default, &points);
    }
    assert!(!default);
    out(points.len().to_string());

    for _ in 2..50 {
        (default, points) = step(&rule, default, &points);
    }
    assert!(!default);
    out(points.len().to_string());
}

fn step(
    rule: &[bool],
    default: bool, points: &HashSet<(i32, i32)>,
) -> (bool, HashSet<(i32, i32)>) {
    assert_eq!(rule.len(), 512);
    let new_default = rule[511 * default as usize];
    let mut min_i = i32::MAX;
    let mut min_j = i32::MAX;
    let mut max_i = i32::MIN;
    let mut max_j = i32::MIN;
    for &(i, j) in points {
        min_i = min_i.min(i);
        min_j = min_j.min(j);
        max_i = max_i.max(i);
        max_j = max_j.max(j);
    }
    let mut new_points = HashSet::default();
    for i in min_i - 1 .. max_i + 2 {
        for j in min_j - 1 .. max_j + 2 {
            let mut idx = 0;
            for ii in i - 1 .. i + 2 {
                for jj in j - 1 .. j + 2 {
                    idx *= 2;
                    if points.contains(&(ii, jj)) {
                        idx += (!default) as usize;
                    } else {
                        idx += default as usize;
                    }
                }
            }
            if rule[idx] != new_default {
                new_points.insert((i, j));
            }
        }
    }
    (new_default, new_points)
}

fn to_bool(c: u8) -> bool {
    match c {
        b'.' => false,
        b'#' => true,
        _ => panic!(),
    }
}
