struct Solution;
impl Solution {
    pub fn kth_character(k: i64, operations: Vec<i32>) -> char {
        Self::inner(k as u64 - 1, &operations, 0)
    }

    fn inner(k: u64, ops: &[i32], shifts: usize) -> char {
        if ops.is_empty() {
            // println!("shifts {shifts}");
            return ((shifts % 26) as u8 + b'a') as char;
        }

        let ops_no_last = &ops[..ops.len() - 1];
        // println!("size = {}", 2u64.pow(ops.len() as u32 - 1));
        if ops.len() < 64 && 2u64.pow(ops.len() as u32 - 1) < dbg!(k) {
            let offset = 2u64.pow(ops.len() as u32 - 1);
            // println!("1");
            Self::inner(
                k - offset,
                ops_no_last,
                shifts + *ops.last().unwrap() as usize,
            )
        } else {
            // println!("2");
            Self::inner(k, ops_no_last, shifts)
        }
    }
}
