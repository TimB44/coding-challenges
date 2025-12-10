use std::io::{stdin, BufRead};

enum Op {
    Add,
    Mult,
}
fn main() {
    let lines: Vec<String> = stdin().lock().lines().map(Result::unwrap).collect();
    let grid: Vec<Vec<u64>> = lines[0..lines.len() - 1]
        .iter()
        .map(|l| l.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect();

    let ops: Vec<_> = lines
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| if s == "+" { Op::Add } else { Op::Mult })
        .collect();

    let results = grid[1..].iter().fold(grid[0].clone(), |mut acc, row| {
        for ((n, op), res) in row.iter().zip(ops.iter()).zip(acc.iter_mut()) {
            match op {
                Op::Add => *res = *res + *n,
                Op::Mult => *res = *res * *n,
            }
        }
        acc
    });

    let final_sum: u64 = results.iter().sum();
    println!("Final sum = {final_sum}");
}
