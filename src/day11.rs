use std::{collections::HashMap, io::stdin};

const START_STATE: &str = "svr";
const END_STATE: &str = "out";
const TARGET_1: &str = "dac";
const TARGET_2: &str = "fft";

fn count_paths<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    cur: &'a str,
    cache: &mut HashMap<&'a str, [[usize; 2]; 2]>,
) -> [[usize; 2]; 2] {
    if cur == END_STATE {
        [[1, 0], [0, 0]]
    } else if let Some(ans) = cache.get(cur) {
        *ans
    } else {
        let mut ans = graph[cur]
            .iter()
            .map(|new_state| count_paths(graph, new_state, cache))
            .fold([[0; 2]; 2], |acc, elem| {
                [
                    [acc[0][0] + elem[0][0], acc[0][1] + elem[0][1]],
                    [acc[1][0] + elem[1][0], acc[1][1] + elem[1][1]],
                ]
            });
        if cur == TARGET_1 {
            ans[1][0] += ans[0][0];
            ans[1][1] += ans[0][1];
            ans[0][0] = 0;
            ans[0][1] = 0;
        } else if cur == TARGET_2 {
            ans[0][1] += ans[0][0];
            ans[1][1] += ans[1][0];
            ans[0][0] = 0;
            ans[1][0] = 0;
        }

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
    let ans = count_paths(&graph, START_STATE, &mut HashMap::new())[1][1];
    println!("Path count = {ans}")
}
