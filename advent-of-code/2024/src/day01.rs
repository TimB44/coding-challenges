use std::{collections::HashMap, io::stdin};

fn main() {
    let mut nums1: Vec<u32> = Vec::new();
    let mut nums2: HashMap<u32, u32> = HashMap::new();
    for line in stdin().lines().map(Result::unwrap) {
        let mut line_iter = line.split_whitespace();
        nums1.push(line_iter.next().unwrap().parse().unwrap());
        nums2
            .entry(line_iter.next().unwrap().parse().unwrap())
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    nums1.sort_unstable();

    let output: u32 = nums1
        .into_iter()
        .map(|n1| n1 * nums2.get(&n1).unwrap_or(&0))
        .sum();

    println!("Result is {output}");
}
