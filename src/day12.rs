use std::io::stdin;

fn main() {
    let ok = stdin()
        .lines()
        .map(Result::unwrap)
        .filter(|line| line.contains('x'))
        .filter(|l| {
            let mut iter = l
                .split(|c: char| !c.is_ascii_digit())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<usize>().unwrap());
            iter.next().unwrap() * iter.next().unwrap() > 8 * iter.sum::<usize>()
        })
        .count();
    println!("Number that fit = {ok}");
}
