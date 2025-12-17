use std::cmp;
struct Solution;

const CM: i32 = 3_628_800;
impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        let numerator = expression
            .split(&['-', '+'])
            .filter(|s| !s.is_empty())
            .map(|s| {
                let mut i = s.split('/').map(|s| s.parse::<i32>().unwrap());
                i.next().unwrap() * (CM / i.next().unwrap())
            })
            .zip(
                match expression.chars().next().unwrap() {
                    '-' => Some(-1),
                    _ => Some(1),
                }
                .into_iter()
                .chain(expression.chars().skip(1).filter_map(|c| match c {
                    '+' => Some(1),
                    '-' => Some(-1),
                    _ => None,
                })),
            )
            .map(|(num, sign)| sign * num)
            .sum();
        let (num, den) = reduce(numerator, CM);
        format!("{}/{}", num, den)
    }
}

fn reduce(numerator: i32, denominator: i32) -> (i32, i32) {
    let mut min = cmp::min(numerator.abs(), denominator.abs());
    let mut max = cmp::max(numerator.abs(), denominator.abs());
    while max % min != 0 {
        (min, max) = (max % min, min);
    }
    (numerator / min, denominator / min)
}
