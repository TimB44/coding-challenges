struct Solution;

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        s.as_bytes()
            .chunks_exact(2)
            .map(|c| if c[0] != c[1] { 1 } else { 0 })
            .sum()
    }
}
