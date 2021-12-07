pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut counts = [0; 9];
    for s in input.trim_end().split(',') {
        counts[s.parse::<usize>().unwrap()] += 1;
    }
    for _ in 0..80 {
        let z = counts[0];
        counts.rotate_left(1);
        counts[6] += z;
    }
    out(counts.iter().sum::<i32>().to_string());
}
