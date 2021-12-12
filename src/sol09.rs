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

    let mut visited: Vec<bool> = hmap.iter().map(|&h| h == b'9').collect();
    let mut sizes = vec![];
    let mut q = vec![];
    for idx in 0 .. w * h {
        if visited[idx] {
            continue;
        }
        visited[idx] = true;
        let mut size = 1;
        q.push(idx);
        while let Some(idx) = q.pop() {
            for delta in [-1, 1, w as isize, -(w as isize)] {
                let idx2 = (idx as isize + delta) as usize;
                if !visited[idx2] {
                    visited[idx2] = true;
                    q.push(idx2);
                    size += 1;
                }
            }
        }
        sizes.push(size);
    }
    let t = sizes.len() - 3 - 1;
    let (_, _, last3) = sizes.select_nth_unstable(t);
    let mut part2 = 1;
    for &mut size in last3 {
        part2 *= size;
    }
    out(part2.to_string());
}
