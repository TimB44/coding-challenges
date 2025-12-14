use std::{
    collections::{BTreeSet, HashMap},
    io::stdin,
};

fn area([x1, y1]: [usize; 2], [x2, y2]: [usize; 2]) -> usize {
    let width = x1.abs_diff(x2) + 1;
    let height = y1.abs_diff(y2) + 1;

    width * height
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
    let max_area = points
        .iter()
        .flat_map(|p1| points.iter().map(move |p2| area(*p1, *p2)))
        .max()
        .unwrap();

    println!("Max area = {max_area}");
}
