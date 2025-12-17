use std::collections::VecDeque;

#[derive(Copy, Clone, Debug)]
enum Period {
    One,
    Two,
    Five,
}
fn n_to_period(n: i32) -> Period {
    match n {
        1 | 3 | 7 | 9 => Period::One,
        2 | 4 | 6 | 8 => Period::Two,
        5 => Period::Five,
        _ => unreachable!(),
    }
}

fn minimize_digit(n: u8, p: Period) -> u8 {
    match p {
        Period::One => 0,
        Period::Two if n % 2 == 0 => 0,
        Period::Two => 1,
        Period::Five if n >= 5 => n - 5,
        Period::Five => n,
    }
}

struct Solution;
impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let b = b as usize;
        let mut shifted: usize = 0;
        let n = s.len();
        let mut min: VecDeque<u8> = s.chars().map(|c| c as u8 - b'0').collect();
        let mut nums: VecDeque<u8> = s.chars().map(|c| c as u8 - b'0').collect();
        let p = n_to_period(a);

        loop {
            nums = nums.into_iter().map(|n| minimize_digit(n, p)).collect();
            if nums < min {
                min = nums.clone();
            }
            nums.rotate_right(b);
            shifted = (shifted + b) % n;

            if shifted == 0 {
                break;
            }
        }

        min.into_iter().map(|n| (n + b'0') as char).collect()
    }
}
