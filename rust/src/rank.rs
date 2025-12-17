struct Solution {}
impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let n = arr.len();
        let mut sorted: Vec<(i32, usize)> =
            arr.into_iter().enumerate().map(|(i, v)| (v, i)).collect();
        sorted.sort_unstable();
        let mut out = vec![0; n];
        let mut prev = None;
        let mut cur_rank = 0;
        for (val, i) in sorted {
            if Some(val) != prev {
                cur_rank += 1;
            }
            out[i] = cur_rank;
            prev = Some(val);
        }
        out
    }
}
