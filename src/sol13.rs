pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut it = input.split_terminator('\n');
    let mut dots = vec![];
    loop {
        let line = it.next().unwrap();
        if line.is_empty() {
            break;
        }
        let (x, y) = line.split_once(',').unwrap();
        let x: i32 = x.parse().unwrap();
        let y: i32 = y.parse().unwrap();
        dots.push((x, y));
    }

    let line = it.next().unwrap();
    let line = line.strip_prefix("fold along ").unwrap();
    if let Some(fold_x) = line.strip_prefix("x=") {
        let fold_x = fold_x.parse().unwrap();
        for (x, _y) in &mut dots {
            assert_ne!(*x, fold_x);
            if *x > fold_x {
                *x = 2 * fold_x - *x;
            }
        }
    } else if let Some(fold_y) = line.strip_prefix("y=") {
        let fold_y = fold_y.parse().unwrap();
        for (_x, y) in &mut dots {
            assert_ne!(*y, fold_y);
            if *y > fold_y {
                *y = 2 * fold_y - *y;
            }
        }
    } else {
        panic!();
    }

    dots.sort_unstable();
    dots.dedup();
    out(dots.len().to_string());
}
