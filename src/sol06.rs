pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut counts = [0i64; 9];
    for s in input.trim_end().split(',') {
        counts[s.parse::<usize>().unwrap()] += 1;
    }
    for day in 1..=256 {
        let z = counts[0];
        counts.rotate_left(1);
        counts[6] += z;
        if day == 80 || day == 256 {
            out(counts.iter().sum::<i64>().to_string());
        }
    }
}
