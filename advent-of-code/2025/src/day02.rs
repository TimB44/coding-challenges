use std::io::stdin;

fn is_prime(n: u32) -> bool {
    !(2..=(n.isqrt())).any(|m| n.is_multiple_of(m))
}

struct PrimeFactors {
    n: u32,
    cur: u32,
}

impl PrimeFactors {
    fn new(n: u32) -> Self {
        Self { n, cur: 1 }
    }
}

impl Iterator for PrimeFactors {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur += 1;
        if self.cur > self.n {
            None
        } else if is_prime(self.cur) && self.n.is_multiple_of(self.cur) {
            Some(self.cur)
        } else {
            self.next()
        }
    }
}

fn is_repeated(n: u64) -> bool {
    let base_10_len = n.ilog10() + 1;
    PrimeFactors::new(base_10_len).any(|parts| {
        let magnitude = 10u64.pow(base_10_len / parts);
        let part = n % magnitude;
        (1..parts).all(|i| part == (n / magnitude.pow(i)) % magnitude)
    })
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

    #[test]
    fn is_prime_test() {
        assert!(is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(!is_prime(6));
        assert!(is_prime(7));
        assert!(!is_prime(8));
        assert!(!is_prime(9));
        assert!(!is_prime(10));
    }

    #[test]
    fn prime_iter_tests() {
        assert_eq!(PrimeFactors::new(4).collect::<Vec<_>>(), vec![2]);
        assert_eq!(PrimeFactors::new(6).collect::<Vec<_>>(), vec![2, 3]);
        assert_eq!(PrimeFactors::new(9).collect::<Vec<_>>(), vec![3]);
    }
}
