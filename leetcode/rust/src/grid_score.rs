struct Solution;
use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap, HashSet},
    iter::repeat,
    usize,
};

impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut seen = HashSet::new();
        let mut q = BinaryHeap::from([Reverse((grid[0][0], 0, 0))]);
        let mut can_get_with = BTreeMap::from_iter(queries.iter().copied().zip(repeat(0)));
        while let Some(Reverse((cost, row, col))) = q.pop() {
            if !seen.insert((row, col)) {
                continue;
            }
            if let Some((_, n)) = can_get_with.range_mut((cost + 1)..).next() {
                *n += 1;
            }
            let directions = [(0, 1), (1, 0), (-1, 0), (0, -1)];

            for &(dx, dy) in &directions {
                let new_row = row as isize + dy;
                let new_col = col as isize + dx;
                if new_row < 0
                    || new_col < 0
                    || new_row >= rows as isize
                    || new_col >= cols as isize
                    || seen.contains(&(new_row as usize, new_col as usize))
                {
                    continue;
                }

                q.push(Reverse((
                    cost.max(grid[new_row as usize][new_col as usize]),
                    new_row as usize,
                    new_col as usize,
                )))
            }
        }

        let mut iter = can_get_with.iter_mut();
        let mut last = *iter.next().unwrap().1;
        for (k, v) in iter {
            *v += last;
            last = *v;
        }

        queries
            .iter()
            .map(|max_cost| can_get_with.get(max_cost).unwrap())
            .copied()
            .collect()
    }
}
