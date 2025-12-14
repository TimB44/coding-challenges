use std::{
    collections::{HashSet, VecDeque},
    io::stdin,
};

fn min_path(line: String) -> usize {
    let parts: Vec<_> = line.split_whitespace().collect();
    let edges: Vec<u64> = parts[1..parts.len() - 1]
        .iter()
        .map(|s| {
            s[1..s.len() - 1]
                .split(',')
                .fold(0, |m, n| m | 1 << n.parse::<u64>().unwrap())
        })
        .collect();

    let end_str = parts[0];
    let end: u64 = end_str[1..end_str.len() - 1]
        .bytes()
        .map(|b| if b == b'#' { 1 } else { 0 })
        .rev()
        .fold(0, |m, b| (m << 1) | b);

    let mut seen = HashSet::from([0]);
    let mut q = VecDeque::from([(0, 0)]);

    loop {
        let (state, cost) = q.pop_front().unwrap();
        if state == end {
            return cost;
        }
        for new_state in edges
            .iter()
            .map(|&e| e ^ state)
            .filter(|&new_state| seen.insert(new_state))
        {
            q.push_back((new_state, cost + 1));
        }
    }
}
fn main() {
    let sum: usize = stdin().lines().map(Result::unwrap).map(min_path).sum();

    println!("Path sum = {sum}");
}
