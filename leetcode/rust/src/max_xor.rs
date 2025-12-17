struct Solution;
impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mask = 1 << (maximum_bit - 1);
        let n = nums.len();
        let mut cur_xor = 0;
        let mut out = Vec::with_capacity(n);
        for num in nums {
            cur_xor ^= num;
            out.push((!cur_xor) & mask);
        }

        out.reverse();
        out
    }
}
