pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let (template, rules) = input.split_once("\n\n").unwrap();
    let template = template.as_bytes();
    let rules: Vec<((u8, u8), u8)> = rules.split_terminator('\n')
        .map(|rule| {
            let (pair, ins) = rule.split_once(" -> ").unwrap();
            let &[p1, p2] = pair.as_bytes() else { panic!() };
            let &[ins] = ins.as_bytes() else { panic!() };
            ((p1, p2), ins)
        })
        .collect();

    const N: usize = b'Z' as usize + 1;
    let mut pair_counts = vec![0; N * N];
    for (&a, &b) in template.iter().zip(&template[1..]) {
        pair_counts[a as usize * N + b as usize] += 1;
    }

    for _ in 0..10 {
        let mut new_pair_counts = pair_counts.clone();
        for &((p1, p2), ins) in &rules {
            let k = pair_counts[p1 as usize * N + p2 as usize];
            new_pair_counts[p1 as usize * N + p2 as usize] -= k;
            new_pair_counts[p1 as usize * N + ins as usize] += k;
            new_pair_counts[ins as usize * N + p2 as usize] += k;
        }
        pair_counts = new_pair_counts;
    }
    let mut char_counts = vec![0; N];
    for p1 in 0..N {
        for p2 in 0..N {
            char_counts[p1] += pair_counts[p1 * N + p2];
            char_counts[p2] += pair_counts[p1 * N + p2];
        }
    }
    char_counts[*template.first().unwrap() as usize] += 1;
    char_counts[*template.last().unwrap() as usize] += 1;
    for k in &mut char_counts {
        assert_eq!(*k % 2, 0);
        *k /= 2;
    }
    let max = *char_counts.iter().max().unwrap();
    let min = *char_counts.iter().filter(|&&k| k > 0).min().unwrap();
    out((max - min).to_string());
}
