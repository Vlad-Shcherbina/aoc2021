pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let h = 2 + input.split_terminator('\n').count();
    let w = 2 + input.split_terminator('\n').next().unwrap().len();
    let mut state = vec![-128i8; w * h];
    for (i, line) in input.split_terminator('\n').enumerate() {
        assert_eq!(2 + line.len(), w);
        for (j, c) in line.bytes().enumerate() {
            assert!((b'0'..b'9').contains(&c));
            state[(i + 1) * w + j + 1] = (c - b'0') as i8;
        }
    }
    let mut num_flashes = 0;
    for _ in 0..100 {
        for j in 0..w {
            state[j] = -128;
            state[(h - 1) * w + j] = -128;
        }
        for i in 0..h {
            state[i * w] = 0;
            state[i * w + h - 1] = -128;
        }

        let mut to_flash = vec![];
        for (idx, s) in state.iter_mut().enumerate() {
            *s += 1;
            if *s > 9 {
                to_flash.push(idx);
                *s = -128;
            }
        }
        while let Some(idx) = to_flash.pop() {
            num_flashes += 1;
            for i in 0..3 {
                for j in 0..3 {
                    let idx2 = idx - w - 1 + i * w + j;
                    state[idx2] += 1;
                    if state[idx2] > 9 {
                        to_flash.push(idx2);
                        state[idx2] = -128;
                    }
                }
            }
        }
        for s in &mut state {
            if *s < 0 {
                *s = 0;
            }
        }
    }
    out(num_flashes.to_string());
}
