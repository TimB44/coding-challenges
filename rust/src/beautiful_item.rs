use std::collections::btree_map::BTreeMap;
struct Solution;
impl Solution {
    pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut max_b = 0;
        let mut map = BTreeMap::new();
        items.sort_unstable_by_key(|item| item[0]);
        for item in items {
            let p = item[0];
            let b = item[1];
            if b > max_b {
                map.insert(p, b);
                max_b = b;
            }
        }

        queries
            .into_iter()
            .map(|max_cost| {
                map.range(..=max_cost)
                    .next_back()
                    .map(|item| *item.1)
                    .unwrap_or(0)
            })
            .collect()
    }
}
