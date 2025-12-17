struct Solution;
impl Solution {
    pub fn prime_sub_operation(nums: Vec<i32>) -> bool {
        let mut prime_less: Vec<i32> = vec![1; *nums.iter().max().unwrap() as usize + 1];
        prime_less[0] = -1;
        prime_less[1] = -1;
        prime_less[2] = -1;

        for i in 3..prime_less.len() {
            if is_prime(i as i32 - 1) {
                prime_less[i] = i as i32 - 1;
            } else {
                prime_less[i] = prime_less[i - 1];
            }
        }

        let mut prev = 0;
        for num in nums {
            if num < prev {
                return false;
            }

            prev = num - (prime_less[(num - prev - 1) as usize]);
        }

        true
    }
}

fn is_prime(num: i32) -> bool {
    let mut cur = 2;
    let max = (num as f64).sqrt().floor() as i32;
    while cur <= max {
        if num % cur == 0 {
            return false;
        }
        cur += 1;
    }
    true
}
