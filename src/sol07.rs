pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut xs: Vec<i32> = input.trim_end().split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let i = xs.len() / 2;
    xs.select_nth_unstable(i);
    out(xs.iter().map(|&x| (x - xs[i]).abs()).sum::<i32>().to_string());
}
