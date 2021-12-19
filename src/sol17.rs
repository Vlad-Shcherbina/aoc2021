pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let (x_range, y_range) = input.trim_end()
        .strip_prefix("target area: x=").unwrap()
        .split_once(", y=").unwrap();
    let x_range = parse_range(x_range);
    let y_range = parse_range(y_range);
    let mut part1 = 0;
    for vx0 in 1 ..= *x_range.end() {
        for vy0 in 1 ..= -*y_range.start() {
            let mut vx = vx0;
            let mut vy = vy0;
            let mut x = 0;
            let mut y = 0;
            let mut max_y = 0;
            loop {
                max_y = max_y.max(y);
                if x_range.contains(&x) && y_range.contains(&y) {
                    part1 = part1.max(max_y);
                    break;
                }
                if x > *x_range.end() || y < *y_range.start() {
                    break;
                }
                x += vx;
                y += vy;
                vy -= 1;
                vx -= vx.signum();
            }
        }
    }
    out(part1.to_string());
}

fn parse_range(r: &str) -> std::ops::RangeInclusive<i32> {
    let (min, max) = r.split_once("..").unwrap();
    min.parse().unwrap() ..= max.parse().unwrap()
}
