use std::{collections::HashMap, io::stdin};

const START_STATE: &str = "you";
const END_STATE: &str = "out";

fn count_paths<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    cur: &'a str,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if cur == END_STATE {
        1
    } else if let Some(ans) = cache.get(cur) {
        *ans
    } else {
        let ans = graph[cur]
            .iter()
            .map(|new_state| count_paths(graph, new_state, cache))
            .sum();

        cache.insert(cur, ans);
        ans
    }
}

fn main() {
    let lines: Vec<String> = stdin().lines().map(Result::unwrap).collect();
    let graph: HashMap<&str, Vec<&str>> = lines
        .iter()
        .map(|l| {
            let mut iter = l.split_whitespace();
            let start = iter.next().unwrap();
            let edges: Vec<&str> = iter.collect();
            (&start[0..start.len() - 1], edges)
        })
        .collect();
    let ans = count_paths(&graph, START_STATE, &mut HashMap::new());
    println!("Path count = {ans}")
}
