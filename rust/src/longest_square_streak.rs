struct Solution;
use std::{collections::HashMap, iter::repeat};

impl Solution {
    pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
        let mut nums_dict = HashMap::from_iter(nums.iter().map(|&n| n as i64).zip(repeat(None)));

        fn max_streak(num: i64, nums: &mut HashMap<i64, Option<i32>>) -> i32 {
            let streak = nums.get(&num);
            match streak {
                Some(&None) => {
                    let max_streak = max_streak(num * num, nums) + 1;
                    nums.insert(num, Some(max_streak));
                    max_streak
                }
                Some(&Some(streak)) => streak,
                None => 0,
            }
        }

        nums.iter()
            .map(|&num| max_streak(num as i64, &mut nums_dict))
            .max()
            .map(|s| if s == 1 { -1 } else { s })
            .unwrap()
    }
}
//impl Solution {
//    pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
//        let mut items: HashMap<i32, Vec<(usize, i32)>> = HashMap::new();
//        for (index, &num) in nums.iter().enumerate().rev() {
//            items
//                .entry(num)
//                .and_modify(|list| list.push((index, num)))
//                .or_insert(vec![(index, num)]);
//        }
//        let mut already_seen = vec![1; nums.len()];
//        nums.iter()
//            .enumerate()
//            .map(|(index, &num)| {
//                assert_eq!(items.get_mut(&num).unwrap().pop(), Some((index, num)));
//                if already_seen[index] != 1 {
//                    return already_seen[index];
//                }
//
//                let mut cur_num = num * num;
//                let mut cur_streak = 1;
//
//                while let Some(&(index, _)) = items.get(&cur_num).map(|l| l.last()).flatten() {
//                    cur_streak += 1;
//                    cur_num = cur_num * cur_num;
//                }
//                let mut streak = cur_streak;
//
//                while let Some(&(index, _)) = items.get(&cur_num).map(|l| l.last()).flatten() {
//                    streak -= 1;
//                    already_seen[index] = streak;
//                    cur_num = cur_num * cur_num;
//                }
//                cur_streak
//            })
//            .max()
//            .unwrap()
//    }
//}
