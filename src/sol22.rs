pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut buf = vec![false; 101 * 101 * 101];
    for line in input.split_terminator('\n') {
        let (is_on, line) = line.split_once(' ').unwrap();
        let is_on = match is_on {
            "on" => true,
            "off" => false,
            _ => panic!(),
        };
        let line = line.strip_prefix("x=").unwrap();
        let (x_range, line) = line.split_once(",y=").unwrap();
        let (y_range, z_range) = line.split_once(",z=").unwrap();
        let x_range = parse_range(x_range);
        let y_range = parse_range(y_range);
        let z_range = parse_range(z_range);

        for x in x_range.0.max(-50) ..= x_range.1.min(50) {
            for y in y_range.0.max(-50) ..= y_range.1.min(50) {
                for z in z_range.0.max(-50) ..= z_range.1.min(50) {
                    let idx = ((x + 50) * 101 + y + 50) * 101 + z + 50;
                    buf[idx as usize] = is_on;
                }
            }
        }
    }
    let part1 = buf.iter().filter(|&&v| v).count();
    out(part1.to_string());
}

fn parse_range(s: &str) -> (i32, i32) {
    let (min, max) = s.split_once("..").unwrap();
    (min.parse().unwrap(), max.parse().unwrap())
}
