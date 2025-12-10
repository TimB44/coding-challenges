use std::io::stdin;

fn is_repeated(n: u64) -> bool {
    let base_10_len = n.ilog10() + 1;
    if !base_10_len.is_multiple_of(2) {
        return false;
    }

    let magnitude = 10u64.pow(base_10_len / 2);

    let upper_part = n / magnitude;
    let lower_part = n % magnitude;
    upper_part == lower_part
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let ranges: Vec<[u64; 2]> = line
        .trim()
        .split(',')
        .map(|range| {
            range
                .split('-')
                .map(|s| s.parse::<u64>())
                .collect::<Result<Vec<_>, _>>()
                .unwrap()
                .try_into()
                .unwrap()
        })
        .collect();

    let id_sum: u64 = ranges
        .into_iter()
        .flat_map(|[l, h]| (l..=h).filter(|n| is_repeated(*n)))
        .sum();

    println!("Sum = {id_sum}")
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn is_repeated_tests() {
        assert!(is_repeated(11));
        assert!(is_repeated(1111));
        assert!(is_repeated(12341234));
        assert!(!is_repeated(123));
        assert!(!is_repeated(12));
        assert!(!is_repeated(1221));
    }
}
