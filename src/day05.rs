use std::{
    collections::BTreeMap,
    io::{stdin, BufRead},
};

struct RangeSet {
    ranges: BTreeMap<u64, u64>,
}

impl RangeSet {
    fn new(mut ranges: Vec<[u64; 2]>) -> Self {
        ranges.sort();
        let mut map: BTreeMap<u64, u64> = BTreeMap::new();
        for [l, h] in ranges {
            if let Some((_, cur_h)) = map.range_mut(..=l).next_back() &&l <= *cur_h {
                *cur_h = (*cur_h).max(h);
            } else {
                map.insert(l, h);
            }
        }

        Self { ranges: map }
    }

    fn in_range(&self, q: u64) -> bool {
        self.ranges
            .range(..=q)
            .next_back()
            .is_some_and(|(_, h)| q <= *h)
    }

    fn count_ranges(&self) -> u64 {
        self.ranges.iter().map(|(l, h)| h - l + 1).sum()
    }
}

fn main() {
    let ranges: Vec<[u64; 2]> = stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .take_while(|l| !l.is_empty())
        .map(|l| {
            l.split('-')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();

    let range_set = RangeSet::new(ranges);

    let range_count = range_set.count_ranges();
    println!("Item in all ranges = {range_count}");
}
