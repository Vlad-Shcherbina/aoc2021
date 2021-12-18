#[derive(Clone, Copy, PartialEq)]
enum CaveType {
    Terminal,  // start or end
    Small,
    Big,
}
use CaveType::*;

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
    let cave_type: Vec<CaveType> = names.iter()
        .map(|&name|
            if name == "start" || name == "end" {
                Terminal
            } else if name.chars().next().unwrap().is_ascii_lowercase() {
                Small
            } else {
                Big
            }
        ).collect();
    let mut visited = vec![false; names.len()];

    visited[start] = true;
    let mut num_paths = 0;

    rec(&cave_type, &adj, end, start, &mut visited, &mut num_paths);
    out(num_paths.to_string());
}

fn rec(
    cave_type: &[CaveType], adj: &[Vec<usize>], end: usize,
    last: usize,
    visited: &mut Vec<bool>, num_paths: &mut usize,
) {
    if last == end {
        *num_paths += 1;
    }
    for &u in &adj[last] {
        if cave_type[u] != Big {
            if visited[u] {
                continue;
            }
            visited[u] = true;
        }

        rec(cave_type, adj, end, u, visited, num_paths);

        if cave_type[u] != Big {
            visited[u] = false;
        }
    }
}
