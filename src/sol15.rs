pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let h = 2 + input.split_terminator('\n').count();
    let w = 2 + input.split_terminator('\n').next().unwrap().len();
    let mut risk = vec![0; w * h];
    for (i, line) in input.split_terminator('\n').enumerate() {
        assert_eq!(2 + line.len(), w);
        for (j, c) in line.bytes().enumerate() {
            assert!((b'0'..=b'9').contains(&c));
            risk[(i + 1) * w + (j + 1)] = (c - b'0') as i32;
        }
    }

    out(dijkstra(h, w, &risk).to_string());

    let w2 = (w - 2) * 5 + 2;
    let h2 = (w - 2) * 5 + 2;
    let mut risk2 = vec![0; w2 * h2];
    for ii in 0..5 {
        for i in 1 .. h - 1 {
            for jj in 0..5 {
                for j in 1 .. w - 1 {
                    let r = (risk[i * w + j] + (ii + jj) as i32 + 8) % 9 + 1;
                    risk2[w2 * (ii * (h - 2) + i) + (jj * (w - 2) + j)] = r;
                }
            }
        }
    }
    out(dijkstra(h2, w2, &risk2).to_string());
}

fn dijkstra(h: usize, w: usize, risk: &[i32]) -> i32 {
    let mut dist = vec![i32::MAX; w * h];
    for i in 0..h {
        dist[i * w] = 0;
        dist[i * w + w - 1] = 0;
    }
    for j in 0..w {
        dist[j] = 0;
        dist[j + w * (h - 1)] = 0;
    }

    let mut qs = vec![vec![]; 10];
    qs[0].push(1 + w);
    dist[1 + w] = 0;

    let target = w - 2 + (h - 2) * w;

    for step in 0.. {
        let qq = std::mem::take(&mut qs[step % 10]);
        for &u in &qq {
            if u == target {
                return dist[u];
            }
            for v in [u - 1, u + 1, u - w, u + w] {
                let d = dist[u] + risk[v];
                if d < dist[v] {
                    dist[v] = d;
                    qs[d as usize % 10].push(v);
                }
            }
        }
        assert_eq!(qs[step % 10].capacity(), 0);
        qs[step % 10] = qq;
        qs[step % 10].clear();
    }
    unreachable!()
}
