pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let input = input.trim_end();
    let mut bits = vec![0u8; input.len() * 4];
    for (i, c) in input.bytes().enumerate() {
        let c = if (b'0'..=b'9').contains(&c) {
            c - b'0'
        } else if (b'A'..=b'F').contains(&c) {
            c - b'A' + 10
        } else { panic!() };
        assert!(c < 16);
        bits[i * 4] = c / 8;
        bits[i * 4 + 1] = c / 4 % 2;
        bits[i * 4 + 2] = c / 2 % 2;
        bits[i * 4 + 3] = c % 2;
    }
    let mut bits: &[u8] = &bits;
    let mut version_sum = 0;
    read_packet(&mut bits, &mut version_sum);
    out(version_sum.to_string());
    for &bit in bits {
        assert_eq!(bit, 0);
    }
}

fn read_num(len: i32, bits: &mut &[u8]) -> i32 {
    let mut res = 0;
    for _ in 0..len {
        res *= 2;
        res += bits[0] as i32;
        *bits = &bits[1..];
    }
    res
}

fn read_packet(bits: &mut &[u8], version_sum: &mut i32) {
    let v = read_num(3, bits);
    *version_sum += v;
    let t = read_num(3, bits);
    if t == 4 { // literal value
        let mut _res = 0i64;
        loop {
            let group = read_num(5, bits);
            _res *= 16;
            _res += group as i64 % 16;
            if group < 16 {
                break;
            }
        }
    } else {
        let indicator = read_num(1, bits);
        if indicator == 0 {
            let len = read_num(15, bits) as usize;
            let target_remaining_len = bits.len() - len;
            while bits.len() > target_remaining_len {
                read_packet(bits, version_sum);
            }
            assert_eq!(bits.len(), target_remaining_len);
        } else {
            let num = read_num(11, bits);
            for _ in 0..num {
                read_packet(bits, version_sum);
            }
        }
    }
}
