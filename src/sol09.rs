pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let lines: Vec<&[u8]> = input.split_terminator('\n')
        .map(|line| line.as_bytes())
        .collect();
    let h = lines.len();
    let w = lines[0].len();
    for line in &lines {
        assert_eq!(line.len(), w);
    }
    let mut part1 = 0;
    for i in 0..h {
        for j in 0..w {
            let c = lines[i][j];
            if i > 0 && lines[i - 1][j] <= c {
                continue;
            }
            if i + 1 < h && lines[i + 1][j] <= c {
                continue;
            }
            if j > 0 && lines[i][j - 1] <= c {
                continue;
            }
            if j + 1 < w && lines[i][j + 1] <= c {
                continue;
            }
            part1 += c as i32 - '0' as i32 + 1;
        }
    }
    out(part1.to_string());
}
