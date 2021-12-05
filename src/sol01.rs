pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let depths: Vec<i32> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut cnt = 0;
    for i in 1..depths.len() {
        if depths[i - 1] < depths[i] {
            cnt += 1;
        }
    }
    out(cnt.to_string());
}