pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut names: Vec<&str> = vec![];
    let mut adj: Vec<Vec<usize>> = vec![];

    fn find_name<'a>(name: &'a str, names: &mut Vec<&'a str>, adj: &mut Vec<Vec<usize>>) -> usize {
        match names.iter().position(|&s| s == name) {
            Some(i) => i,
            None => {
                names.push(name);
                adj.push(vec![]);
                names.len() - 1
            }
        }
    }

    for line in input.split_terminator('\n') {
        let (left, right) = line.split_once('-').unwrap();
        let left = find_name(left, &mut names, &mut adj);
        let right = find_name(right, &mut names, &mut adj);
        adj[left].push(right);
        adj[right].push(left);
    }

    let start = find_name("start", &mut names, &mut adj);
    let end = find_name("end", &mut names, &mut adj);
    let is_small: Vec<bool> = names.iter()
        .map(|name| name.chars().next().unwrap().is_ascii_lowercase())
        .collect();
    let mut visited = vec![false; names.len()];

    visited[start] = true;
    let mut path = vec![start];
    let mut num_paths = 0;

    rec(&is_small, &adj, end, &mut path, &mut visited, &mut num_paths);
    out(num_paths.to_string());
}

fn rec(
    is_small: &[bool], adj: &[Vec<usize>], end: usize,
    path: &mut Vec<usize>, visited: &mut Vec<bool>, num_paths: &mut usize,
) {
    let last = *path.last().unwrap();
    if last == end {
        *num_paths += 1;
    }
    for &u in &adj[last] {
        if is_small[u] {
            if visited[u] {
                continue;
            }
            visited[u] = true;
        }
        path.push(u);

        rec(is_small, adj, end, path, visited, num_paths);

        let u2 = path.pop().unwrap();
        assert_eq!(u, u2);
        if is_small[u] {
            visited[u] = false;
        }
    }
}
