use fxhash::FxHashMap as HashMap;

pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut it = input.split_terminator('\n');
    let line = it.next().unwrap();
    let start1: i32 = line.strip_prefix("Player 1 starting position: ").unwrap().parse().unwrap();
    let line = it.next().unwrap();
    let start2: i32 = line.strip_prefix("Player 2 starting position: ").unwrap().parse().unwrap();
    assert!(it.next().is_none());

    let mut scores = [0, 0];
    let mut positions = [start1, start2];
    let mut num_dice_rolls = 0;
    let mut dice = 1;
    for t in 0.. {
        let player = t % 2;
        for _ in 0..3 {
            positions[player] += dice;
            dice = dice % 100 + 1;
            num_dice_rolls += 1;
        }
        positions[player] = (positions[player] + 10 - 1) % 10 + 1;
        scores[player] += positions[player];
        if scores[player] >= 1000 {
            out((scores[1 - player] * num_dice_rolls).to_string());
            break;
        }
    }

    let mut win_cnt = [0, 0];
    let mut cnts = HashMap::default();
    cnts.insert(([0, 0], [start1, start2]), 1u64);
    for t in 0.. {
        if cnts.is_empty() {
            break;
        }
        let player = t % 2;
        let mut new_cnts = HashMap::default();
        for ((scores, poss), cnt) in cnts {
            for d1 in 1..4 {
                for d2 in 1..4 {
                    for d3 in 1..4 {
                        let mut new_poss = poss;
                        new_poss[player] += d1 + d2 + d3;
                        new_poss[player] = (new_poss[player] + 10 - 1) % 10 + 1;
                        let mut new_scores = scores;
                        new_scores[player] += new_poss[player];
                        if new_scores[player] >= 21 {
                            win_cnt[player] += cnt;
                        } else {
                            *new_cnts.entry((new_scores, new_poss)).or_default() += cnt;
                        }
                    }
                }
            }
        }
        cnts = new_cnts;
    }
    out(win_cnt[0].max(win_cnt[1]).to_string());
}
