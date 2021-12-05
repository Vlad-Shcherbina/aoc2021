pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let depths: Vec<i32> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for width in [1, 3] {
        let mut cnt = 0;
        let mut prev = i32::MAX;
        for w in depths.windows(width) {
            let s: i32 = w.iter().sum();
            if s > prev {
                cnt += 1;
            }
            prev = s;
        }
        out(cnt.to_string());
    }
}
