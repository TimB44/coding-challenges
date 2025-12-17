use std::cmp::min;

struct Solution;
impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let mut stack: Vec<(usize, i32)> = Vec::new();
        for (index, item) in nums.iter().copied().enumerate().rev() {
            match stack.last() {
                Some(&(_, last_item)) if item > last_item => stack.push((index, item)),
                None => stack.push((index, item)),
                _ => (),
            }
        }
        println!("{stack:?}");

        // Used for filtering
        let mut min_so_far = i32::MAX;
        nums.iter()
            .copied()
            .enumerate()
            .filter(|&(_, x)| {
                let ret = x < min_so_far;
                min_so_far = min(min_so_far, x);
                ret
            })
            .map(|(index, item)| {
                let mut last_popped = None;
                while let Some(&(popped_index, popped_item)) = stack.last() {
                    if popped_index <= index || popped_item >= item {
                        last_popped = stack.pop();
                    } else {
                        break;
                    }
                }
                println!(
                    "item = {item}, index = {index} last_popped = {last_popped:?}, stack = {:?}",
                    stack
                );

                last_popped.unwrap_or((index, item)).0 - index
            })
            .max()
            .unwrap() as i32
    }
}
