use std::{
    collections::{BTreeSet, HashMap},
    io::stdin,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum BoardState {
    Unknown,
    Border,
    Outside,
}

fn area([x1, y1]: [usize; 2], [x2, y2]: [usize; 2]) -> usize {
    let width = x1.abs_diff(x2) + 1;
    let height = y1.abs_diff(y2) + 1;

    width * height
}

fn minimize_nums(nums: BTreeSet<usize>) -> (HashMap<usize, usize>, usize) {
    let mut map = HashMap::new();
    map.insert(*nums.first().unwrap(), 0);
    let mut prev_minned = 0;

    for (&prev, &cur) in nums.iter().zip(nums.iter().skip(1)) {
        assert!(prev < cur);

        if prev + 1 == cur {
            prev_minned += 1;
        } else {
            prev_minned += 2;
        }
        map.insert(cur, prev_minned);
    }
    (map, prev_minned + 1)
}

fn fill_outside(grid: &mut [Vec<BoardState>]) {
    for i in 0..grid.len() {
        fill(grid, 0, i);
        fill(grid, grid[0].len() - 1, i);
    }

    for i in 0..grid[0].len() {
        fill(grid, i, 0);
        fill(grid, i, grid.len() - 1);
    }
}

fn fill(grid: &mut [Vec<BoardState>], x: usize, y: usize) {
    if x >= grid[0].len() || y >= grid.len() || grid[y][x] != BoardState::Unknown {
        return;
    }
    grid[y][x] = BoardState::Outside;
    if x != 0 {
        fill(grid, x - 1, y);
    }
    if y != 0 {
        fill(grid, x, y - 1);
    }
    fill(grid, x + 1, y);
    fill(grid, x, y + 1);
}

fn is_inside(grid: &[Vec<BoardState>], x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
    (y1.min(y2)..y1.max(y2))
        .flat_map(|y| (x1.min(x2)..x1.max(x2)).map(move |x| grid[y][x]))
        .all(|bs| matches!(bs, BoardState::Border | BoardState::Unknown))
}

fn main() {
    let points: Vec<[usize; 2]> = stdin()
        .lines()
        .map(|l| {
            l.unwrap()
                .split(',')
                .map(|l| l.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();

    let x_points: BTreeSet<_> = points.iter().map(|p| p[0]).collect();
    let (x_points_map, width) = minimize_nums(x_points);

    let y_points: BTreeSet<_> = points.iter().map(|p| p[1]).collect();
    let (y_points_map, height) = minimize_nums(y_points);

    let minimized_points: Vec<[usize; 2]> = points
        .iter()
        .map(|[x, y]| [x_points_map[x], y_points_map[y]])
        .collect();

    let mut grid = vec![vec![BoardState::Unknown; width]; height];
    for points in minimized_points.windows(2).chain([[
        minimized_points[0],
        minimized_points[minimized_points.len() - 1],
    ]
    .as_slice()])
    {
        assert!(points.len() == 2);
        let [x1, y1] = points[0];
        let [x2, y2] = points[1];
        for y in y1.min(y2)..y1.max(y2) + 1 {
            for x in x1.min(x2)..x1.max(x2) + 1 {
                grid[y][x] = BoardState::Border;
            }
        }
    }

    fill_outside(&mut grid);

    let max_area = points
        .iter()
        .enumerate()
        .flat_map(|(i, p1)| points.iter().take(i).map(move |p2| (*p1, *p2)))
        .filter(|([x1, y1], [x2, y2])| {
            is_inside(
                &grid,
                x_points_map[x1],
                y_points_map[y1],
                x_points_map[x2],
                y_points_map[y2],
            )
        })
        .map(|(p1, p2)| area(p1, p2))
        .max()
        .unwrap();

    println!("Max area = {max_area}");
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::{BTreeSet, HashMap};

    #[test]
    fn minimize_dims_test() {
        assert_eq!(
            minimize_nums(BTreeSet::from([1, 2, 4])),
            (HashMap::from([(1, 0), (2, 1), (4, 3)]), 3)
        );
    }
}
