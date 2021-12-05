use std::cmp::Ordering::{Less, Equal, Greater};

pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut cnt = 0;
    let mut cnt1: Vec<i32> = vec![];
    for line in input.split_terminator('\n') {
        cnt += 1;
        if cnt1.is_empty() {
            cnt1.resize(line.len(), 0);
        }
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                cnt1[i] += 1;
            }
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    for c in cnt1 {
        gamma *= 2;
        epsilon *= 2;
        match (2 * c).cmp(&cnt) {
            Less => epsilon += 1,
            Equal => panic!(),
            Greater => gamma += 1,
        }
    }
    out((gamma * epsilon).to_string());
}
