pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let h = 2 + input.split_terminator('\n').count();
    let w = 2 + input.split_terminator('\n').next().unwrap().len();
    let mut hmap = vec![b'9'; w * h];
    for (i, line) in input.split_terminator('\n').enumerate() {
        assert_eq!(2 + line.len(), w);
        hmap[(i + 1) * w + 1 .. (i + 2) * w - 1].copy_from_slice(line.as_bytes());
    }

    let mut part1 = 0;
    for i in 1 .. h - 1 {
        for j in 1 .. w - 1 {
            let idx = i * w + j;
            let c = hmap[idx];
            if c < hmap[idx - 1]
            && c < hmap[idx + 1]
            && c < hmap[idx - w]
            && c < hmap[idx + w] {
                part1 += c as i32 - b'0' as i32 + 1;
            }
        }
    }
    out(part1.to_string());
}
