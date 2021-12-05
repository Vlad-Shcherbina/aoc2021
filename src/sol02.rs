pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut x = 0;
    let mut y = 0;
    for line in input.split_terminator('\n') {
        let (cmd, arg) = line.split_once(' ').unwrap();
        let arg: i32 = arg.parse().unwrap();
        match cmd {
            "forward" => x += arg,
            "down" => y += arg,
            "up" => y -= arg,
            _ => panic!("{}", line),
        }
    }
    out((x * y).to_string());
}
