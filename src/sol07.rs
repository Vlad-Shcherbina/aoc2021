pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut xs: Vec<i32> = input.trim_end().split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let i = xs.len() / 2;
    xs.select_nth_unstable(i);
    out(xs.iter().map(|&x| (x - xs[i]).abs()).sum::<i32>().to_string());

    let mean = xs.iter().sum::<i32>() / xs.len() as i32;
    let part2 = (mean - 5 .. mean + 5).map(|m|
        xs.iter().map(|&x| {
            let d = (x - m).abs();
            d * (d + 1) / 2
        }).sum::<i32>()
    ).min().unwrap();
    out(part2.to_string());
}
