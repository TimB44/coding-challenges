struct Solution;
impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        if k == 0 || nums[0] >= k {
            return 1;
        }

        let n = nums.len();
        let mut l = 0;
        let mut h = 1;
        let mut smallest_subarray = usize::MAX;
        let mut cur_bit_counts = [0; 32];
        add_bits(nums[0], &mut cur_bit_counts);

        while h < n {
            let cur_or = from_bits(&cur_bit_counts);
            if cur_or < k {
                add_bits(nums[h], &mut cur_bit_counts);
                h += 1;
            }

            loop {
                let cur_or = from_bits(&cur_bit_counts);
                if cur_or < k {
                    break;
                }
                smallest_subarray = smallest_subarray.min(h - l);
                del_bits(nums[l], &mut cur_bit_counts);
                l += 1;
            }
        }

        match smallest_subarray {
            usize::MAX => -1,
            num => num as i32,
        }
    }
}

pub fn from_bits(cur_bits: &[i32; 32]) -> i32 {
    let mut out = 0;
    for &count in cur_bits.iter().rev() {
        out <<= 1;
        if count > 0 {
            out |= 1;
        }
    }
    out
}

pub fn add_bits(num: i32, cur_bits: &mut [i32; 32]) {
    let mut mask = 1;
    for count in cur_bits {
        if mask & num != 0 {
            *count += 1;
        }
        mask <<= 1;
    }
}

pub fn del_bits(num: i32, cur_bits: &mut [i32; 32]) {
    let mut mask = 1;
    for count in cur_bits {
        if mask & num != 0 {
            *count -= 1;
        }
        mask <<= 1;
    }
}
