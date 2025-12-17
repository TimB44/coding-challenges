struct Solution;

impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let nums1_sum: i64 = nums1.iter().copied().map(|n| n as i64).sum();
        let nums1_zero_count = nums1.iter().filter(|n| **n == 0).count();
        let nums1_min_sum = nums1_sum + nums1_zero_count as i64;

        let nums2_sum: i64 = nums2.iter().copied().map(|n| n as i64).sum();
        let nums2_zero_count = nums2.iter().filter(|n| **n == 0).count();
        let nums2_min_sum = nums2_sum + nums2_zero_count as i64;

        if nums1_zero_count == 0 && nums2_min_sum > nums1_sum
            || nums2_zero_count == 0 && nums1_min_sum > nums2_sum
        {
            -1
        } else {
            nums1_min_sum.max(nums2_min_sum)
        }
    }
}
