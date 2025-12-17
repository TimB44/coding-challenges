struct Solution;
impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        if n == 1 {
            assert!(k == 1);
            return '0';
        }

        let length = 2usize.pow(n as u32) - 1;

        let mid = length / 2 + 1;

        assert!(k as usize <= length);
        assert!(k > 0);

        if (k as usize) < mid {
            return Solution::find_kth_bit(n - 1, k);
        } else if k as usize == mid {
            return '1';
        }
        match Solution::find_kth_bit(n - 1, length as i32 - k + 1) {
            '1' => '0',
            '0' => '1',
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::k_th_bit::Solution;

    #[test]
    fn test() {
        assert_eq!('1', Solution::find_kth_bit(3, 2));
    }
}
