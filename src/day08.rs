use std::{collections::HashMap, io::stdin};

struct DisjointSet {
    set: Vec<usize>,
}

impl DisjointSet {
    fn new(size: usize) -> Self {
        Self {
            set: (0..size).collect(),
        }
    }

    fn union(&mut self, l: usize, r: usize) {
        let l_rep = self.rep(l);
        let r_rep = self.rep(r);
        self.set[l_rep] = r_rep;
    }

    fn rep(&mut self, n: usize) -> usize {
        let rep = self.set[n];
        if rep == n {
            n
        } else {
            let rep = self.rep(rep);
            self.set[n] = rep;
            rep
        }
    }
}

fn dist([x1, y1, z1]: [f64; 3], [x2, y2, z2]: [f64; 3]) -> f64 {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2) + (z1 - z2).powi(2)).sqrt()
}

fn main() {
    let ranges: Vec<[f64; 3]> = stdin()
        .lines()
        .map(|l| {
            l.unwrap()
                .split(',')
                .map(|s| s.parse::<f64>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();
    let n = ranges.len();

    let mut edges: Vec<_> = ranges
        .iter()
        .enumerate()
        .flat_map(|(v1, &v1_loc)| {
            ranges
                .iter()
                .enumerate()
                .filter(move |(v2, _)| v1 < *v2)
                .map(move |(v2, &v2_loc)| (v1, v2, dist(v1_loc, v2_loc)))
        })
        .collect();

    edges.sort_unstable_by(|l, r| l.2.total_cmp(&r.2));

    let mut set = DisjointSet::new(n);
    let mut edge_count = 0;
    for (v1, v2, _) in edges {
        if set.rep(v1) != set.rep(v2) {
            edge_count += 1;
        }
        set.union(v1, v2);
        if edge_count == n - 1 {
            let ans = ranges[v1][0] * ranges[v2][0];
            println!("Answer = {ans}");

            break;
        }
    }
}
