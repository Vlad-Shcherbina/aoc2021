pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let digits = "abcfeg cf acdeg acdfg bcdf abdfg abdefg acf abcdefg abcdfg";
    let fingerprints: Vec<_> = digits.split(' ')
        .map(|digit| fingerprint(digits, digit))
        .collect();

    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.split_terminator('\n') {
        let (digits, right) = line.split_once(" | ").unwrap();
        let mut n = 0;
        for s in right.split(' ') {
            let k = s.len();
            if k == 2 || k == 4 || k == 3 || k == 7 {
                part1 += 1;
            }

            n *= 10;
            let f = fingerprint(digits, s);
            n += fingerprints.iter().position(|f1| f1 == &f).unwrap();
        }
        part2 += n;
    }
    out(part1.to_string());
    out(part2.to_string());
}

fn fingerprint(digits: &str, s: &str) -> [u8; 8] {
    let mut res = [0u8; 8];
    for digit in digits.split(' ') {
        let overlap = s.chars().filter(|&c| digit.contains(c)).count();
        res[overlap] += 1;
    }
    res
}
