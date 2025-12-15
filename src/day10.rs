use std::{collections::HashMap, io::stdin};

fn min_path(line: String) -> usize {
    let parts: Vec<_> = line.split_whitespace().collect();
    let end_str = parts[parts.len() - 1];
    let end: Vec<usize> = end_str[1..end_str.len() - 1]
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    let edges: Vec<usize> = parts[1..parts.len() - 1]
        .iter()
        .map(|s| {
            s[1..s.len() - 1]
                .split(',')
                .fold(0, |m, n| m | 1 << n.parse::<usize>().unwrap())
        })
        .collect();

    min_sum(&edges, end, &mut HashMap::new())
}

fn min_sum(edges: &[usize], goal: Vec<usize>, cache: &mut HashMap<Vec<usize>, usize>) -> usize {
    if goal.iter().all(|g| *g == 0) {
        return 0;
    }
    if let Some(res) = cache.get(&goal) {
        return *res;
    }

    let cost = (0..(1 << edges.len()))
        .map(|bit_set: usize| {
            (
                bit_set.count_ones(),
                edges
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| ((bit_set >> i) & 1 == 1))
                    .fold(vec![0; goal.len()], |mut v, (_, n)| {
                        let mut n = *n;
                        let mut i = 0;
                        while n != 0 {
                            v[i] += n & 1;
                            n /= 2;
                            i += 1;
                        }
                        v
                    }),
            )
        })
        .filter(|(_, v)| {
            v.iter()
                .zip(goal.iter())
                .all(|(cur, goal)| cur <= goal && (goal - *cur) % 2 == 0)
        })
        .map(|(cost, next_move)| {
            let next_goal: Vec<usize> = next_move
                .into_iter()
                .zip(goal.iter())
                .map(|(i, &m)| m.saturating_sub(i) / 2)
                .collect::<Vec<_>>();
            (cost as usize).saturating_add(min_sum(edges, next_goal, cache).saturating_mul(2))
        })
        .min()
        .unwrap_or(usize::MAX);
    cache.insert(goal, cost);

    cost
}

fn main() {
    let sum: usize = stdin()
        .lines()
        .map(Result::unwrap)
        .map(min_path)
        .sum::<usize>();

    println!("Path sum = {sum}");
}
