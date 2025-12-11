use std::{
    io::{stdin, BufRead},
    iter::repeat_n,
};

enum Op {
    Add,
    Mult,
}
impl Op {
    fn apply(&self, l: u64, r: u64) -> u64 {
        match self {
            Op::Add => l + r,
            Op::Mult => l * r,
        }
    }
}

fn main() {
    let mut lines: Vec<_> = stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(String::into_bytes)
        .collect();
    let longest = lines.iter().map(|s| s.len()).max().unwrap();
    for line in lines.iter_mut() {
        line.extend(repeat_n(b' ', longest - line.len()));
    }

    let lines = lines;

    let ops: Vec<_> = str::from_utf8(lines.last().unwrap())
        .unwrap()
        .split_whitespace()
        .map(|s| if s == "+" { Op::Add } else { Op::Mult })
        .collect();

    let num_rows = lines.len() - 1;

    let rotated: Vec<_> = (0..longest)
        .flat_map(|col| {
            let lines = &lines;
            (0..num_rows).map(move |row| match lines[row][col] {
                v @ b'0'..=b'9' => Some((v - b'0') as u64),
                _ => None,
            })
        })
        .collect();

    let nums: Vec<_> = rotated.chunks_exact(num_rows).collect();
    let mut num_iter = nums.into_iter();
    let final_sum: u64 = ops
        .into_iter()
        .map(|op| {
            num_iter
                .by_ref()
                .take_while(|&n| n.iter().any(Option::is_some))
                .map(|n| {
                    n.iter()
                        .flatten()
                        .copied()
                        .reduce(|l, r| l * 10 + r)
                        .unwrap()
                })
                .reduce(|l, r| op.apply(l, r))
                .unwrap()
        })
        .sum();

    println!("Final sum = {final_sum}");
}
