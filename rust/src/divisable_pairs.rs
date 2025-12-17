use std::usize;

struct Solution {}
impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let counts =
            arr.into_iter()
                .map(|n| n.rem_euclid(k))
                .fold(vec![0; k as usize], |mut acc, rem| {
                    acc[rem as usize] += 1;
                    acc
                });
        counts[0] % 2 == 0
            && counts
                .iter()
                .skip(1)
                .take((k - 1) as usize / 2)
                .eq(counts.iter().rev().take((k - 1) as usize / 2))
    }
}
