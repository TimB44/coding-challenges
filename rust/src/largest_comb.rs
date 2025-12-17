struct Solution;
impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        (0..32)
            .map(|shift_amount| {
                let mask = 1 << shift_amount;
                candidates.iter().filter(|&&n| n & mask != 0).count()
            })
            .max()
            .unwrap() as i32
    }
}
