pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let lines: Vec<&[u8]> = input.split_terminator('\n')
        .map(|s| s.as_bytes())
        .collect();
    let n = lines[0].len();
    for &line in &lines {
        assert_eq!(line.len(), n);
    }

    let mut gamma = vec![];
    let mut epsilon = vec![];
    for i in 0..n {
        let c = most_common(&lines, i);
        gamma.push(c);
        epsilon.push(flip(c));
    }
    out((parse(&gamma) * parse(&epsilon)).to_string());

    let mut lines_oxy = lines.clone();
    for i in 0..n {
        let c = most_common(&lines_oxy, i);
        lines_oxy.retain(|line| line[i] == c);
        if lines_oxy.len() == 1 {
            break;
        }
    }
    assert_eq!(lines_oxy.len(), 1);

    let mut lines_co2 = lines.clone();
    for i in 0..n {
        let c = flip(most_common(&lines_co2, i));
        lines_co2.retain(|line| line[i] == c);
        if lines_co2.len() == 1 {
            break;
        }
    }
    assert_eq!(lines_co2.len(), 1);

    out((parse(lines_oxy[0]) * parse(lines_co2[0])).to_string());
}

fn most_common(lines: &[&[u8]], idx: usize) -> u8 {
    let mut cnt1 = 0;
    for &line in lines {
        if line[idx] == b'1' {
            cnt1 += 1;
        }
    }
    if 2 * cnt1 >= lines.len() {
        b'1'
    } else {
        b'0'
    }
}

fn flip(c: u8) -> u8 {
    match c {
        b'0' => b'1',
        b'1' => b'0',
        _ => panic!()
    }
}

fn parse(s: &[u8]) -> i32 {
    let mut res = 0;
    for c in s {
        res *= 2;
        res += (c - b'0') as i32;
    }
    res
}
