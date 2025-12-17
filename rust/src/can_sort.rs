struct Solution;
use std::cmp::max;
impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        let mut prev_max = 0;
        let mut cur_max = 0;
        let mut cur_set_count = u32::MAX;

        for n in nums {
            if n.count_ones() != cur_set_count {
                prev_max = cur_max;
                cur_max = n;
                cur_set_count = n.count_ones();
            }
            if n < prev_max {
                return false;
            }
            cur_max = max(cur_max, n);
        }
        true
    }
}
