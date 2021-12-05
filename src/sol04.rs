pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut lines = input.split_terminator('\n');
    let seq: Vec<usize> = lines.next().unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut boards = vec![];
    loop {
        match lines.next() {
            None => break,
            Some(s) => assert_eq!(s, "")
        }
        let mut board = vec![];
        for _ in 0..5 {
            let row: Vec<usize> = lines.next().unwrap().split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            board.push(row);
        }
        boards.push(board);
    }

    let mut idx: Vec<Vec<(usize, usize, usize)>> = vec![vec![]; 101];
    for (i, board) in boards.iter().enumerate() {
        for (j, row) in board.iter().enumerate() {
            for (k, &cell) in row.iter().enumerate() {
                idx[cell].push((i, j, k));
            }
        }
    }

    let mut marked = vec![false; 101];
    let mut row_cnt = vec![vec![0; 5]; boards.len()];
    let mut col_cnt = vec![vec![0; 5]; boards.len()];
    for num in seq {
        assert!(!marked[num]);
        marked[num] = true;
        let mut winning_board = None;
        for &(i, j, k) in &idx[num] {
            row_cnt[i][j] += 1;
            if row_cnt[i][j] == 5 {
                assert_eq!(winning_board, None);
                winning_board = Some(i);
            }
            col_cnt[i][k] += 1;
            if col_cnt[i][k] == 5 {
                assert_eq!(winning_board, None);
                winning_board = Some(i);
            }
        }
        if let Some(q) = winning_board {
            let mut sum = 0;
            for row in &boards[q] {
                for &cell in row {
                    if !marked[cell] {
                        sum += cell;
                    }
                }
            }
            out((sum * num).to_string());
            break;
        }
    }
}
