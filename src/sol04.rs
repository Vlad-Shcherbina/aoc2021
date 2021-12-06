pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut lines = input.split_terminator('\n');
    let seq: Vec<usize> = lines.next().unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut seq_pos = vec![usize::MAX; 101];
    for (i, &s) in seq.iter().enumerate() {
        assert_eq!(seq_pos[s], usize::MAX);
        seq_pos[s] = i;
    }

    let mut boards = vec![];
    loop {
        match lines.next() {
            None => break,
            Some(s) => assert_eq!(s, "")
        }
        let mut board = vec![];
        for _ in 0..5 {
            let row: Vec<usize> = lines.next().unwrap().split_whitespace()
                .map(|s| seq_pos[s.parse::<usize>().unwrap()])
                .collect();
            board.push(row);
        }
        boards.push(board);
    }

    let mut win_times = vec![];
    for (board_no, board) in boards.iter().enumerate() {
        let row_time = board.iter()
            .map(|row|
                *row.iter().max().unwrap())
            .min().unwrap();
        let col_time = (0..5)
            .map(|i|
                board.iter().map(|row| row[i]).max().unwrap())
            .min().unwrap();
        win_times.push((row_time.min(col_time), board_no));
    }

    let part1 = *win_times.iter().min().unwrap();
    let part2 = *win_times.iter().max().unwrap();

    for (t, board_no) in [part1, part2] {
        let board = &boards[board_no];
        let mut sum = 0;
        for row in board {
            for &cell in row {
                if cell > t {
                    sum += seq[cell];
                }
            }
        }
        out((sum * seq[t]).to_string());
    }
}
