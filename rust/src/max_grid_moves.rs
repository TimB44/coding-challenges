struct Solution;
use std::cmp;
impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();

        let mut prev_max = vec![0; rows];
        let mut cur_max = vec![-1; rows];
        let mut max = -1;

        for col in 1..cols {
            for row in 0..rows {
                cur_max[row] = [row.saturating_sub(1), row, row + 1]
                    .iter()
                    .filter(|&&cur_row| {
                        cur_row < rows
                            && prev_max[cur_row] >= 0
                            && grid[cur_row][col - 1] < grid[row][col]
                    })
                    .map(|&r| prev_max[r] + 1)
                    .max()
                    .unwrap_or(-1);
                max = cmp::max(max, cur_max[row]);
            }
            (prev_max, cur_max) = (cur_max, prev_max);
            cur_max.fill(-1);
        }

        if let -1 = max {
            0
        } else {
            max
        }
    }
}
