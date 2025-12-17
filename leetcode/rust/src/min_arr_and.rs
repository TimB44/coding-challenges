struct Solution;
impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let mut n = n as i64 - 1;
        let x = x as i64;
        let mut out = x;
        let mut mask = 1;
        while n != 0 {
            if (x & mask) == 0 {
                out |= (n & 1) * mask;
                n >>= 1;
            }
            mask <<= 1;
        }
        out
    }
}
