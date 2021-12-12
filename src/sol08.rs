pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut cnt = 0;
    for line in input.split_terminator('\n') {
        let (_left, right) = line.split_once(" | ").unwrap();
        for s in right.split(' ') {
            let k = s.len();
            if k == 2 || k == 4 || k == 3 || k == 7 {
                cnt += 1;
            }
        }
    }
    out(cnt.to_string());
}
