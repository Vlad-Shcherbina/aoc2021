pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut scores = [0, 0];
    let mut positions = [0, 0];

    let mut it = input.split_terminator('\n');
    let line = it.next().unwrap();
    positions[0] = line.strip_prefix("Player 1 starting position: ").unwrap().parse().unwrap();
    let line = it.next().unwrap();
    positions[1] = line.strip_prefix("Player 2 starting position: ").unwrap().parse().unwrap();
    assert!(it.next().is_none());

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
}
