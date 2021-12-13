pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut part1 = 0;
    let mut scores = vec![];
    for line in input.split_terminator('\n') {
        let mut stack = vec![];
        let mut corrupted = false;
        for c in line.chars() {
            match closing(c) {
                Some(c1) => stack.push(c1),
                None => {
                    if c != stack.pop().unwrap() {
                        part1 += points(c);
                        corrupted = true;
                        break;
                    }
                }
            }
        }
        if !corrupted {
            let mut score = 0;
            for &c in stack.iter().rev() {
                score *= 5;
                score += points2(c);
            }
            scores.push(score);
        }
    }
    out(part1.to_string());

    assert_eq!(scores.len() % 2, 1);
    let t = scores.len() / 2;
    let (_, middle, _) = scores.select_nth_unstable(t);
    out(middle.to_string());
}

fn closing(c: char) -> Option<char> {
    match c {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        ')' => None,
        ']' => None,
        '}' => None,
        '>' => None,
        _ => panic!(),
    }
}

fn points(c: char) -> i32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!(),
    }
}

fn points2(c: char) -> i64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!(),
    }
}
