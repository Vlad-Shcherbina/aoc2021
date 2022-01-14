pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut state: Vec<Vec<u8>> = input.split_terminator('\n')
        .map(|line| line.as_bytes().to_owned())
        .collect();
    let height = state.len();
    let width = state[0].len();
    for row in &state {
        assert_eq!(row.len(), width);
    }

    let mut step = 0;
    loop {
        let mut moved = false;
        for row in &mut state {
            for j in 0..width {
                let j1 = if j + 1 == width { 0 } else { j + 1 };
                if row[j] == b'>' && row[j1] == b'.' {
                    row[j] = b'm';
                }
            }
            for j in 0..width {
                let j1 = if j + 1 == width { 0 } else { j + 1 };
                if row[j] == b'm' {
                    assert_eq!(row[j1], b'.');
                    row[j] = b'.';
                    row[j1] = b'>';
                    moved = true;
                }
            }
        }

        for i in 0..height {
            let i1 = if i + 1 == height { 0 } else { i + 1 };
            for j in 0..width {
                if state[i][j] == b'v' && state[i1][j] == b'.' {
                    state[i][j] = b'm';
                }
            }
        }
        for i in 0..height {
            let i1 = if i + 1 == height { 0 } else { i + 1 };
            for j in 0..width {
                if state[i][j] == b'm' {
                    assert_eq!(state[i1][j], b'.');
                    state[i][j] = b'.';
                    state[i1][j] = b'v';
                    moved = true;
                }
            }
        }

        step += 1;
        if !moved {
            out(step.to_string());
            return;
        }
    }
}
