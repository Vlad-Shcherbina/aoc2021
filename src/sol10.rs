pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut part1 = 0;
    for line in input.split_terminator('\n') {
        let mut stack = vec![];
        for c in line.chars() {
            match closing(c) {
                Some(c1) => stack.push(c1),
                None => {
                    if c != stack.pop().unwrap() {
                        part1 += points(c);
                        break;
                    }
                }
            }
        }
    }
    out(part1.to_string());
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
