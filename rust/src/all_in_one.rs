use std::collections::{HashMap, HashSet};

struct AllOne {
    key_to_count: HashMap<String, usize>,
    count_to_node: HashMap<usize, ListNode>,
}

struct ListNode {
    prev: usize,
    next: usize,
    num: usize,
    here: HashSet<String>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {
    fn new() -> Self {
        let key_to_count = HashMap::new();
        let mut count_to_node = HashMap::new();

        let first = ListNode {
            prev: 0,
            next: usize::MAX,
            num: 0,
            here: HashSet::new(),
        };

        let last = ListNode {
            prev: 0,
            next: usize::MAX,
            num: 0,
            here: HashSet::new(),
        };

        count_to_node.insert(0, first);
        count_to_node.insert(usize::MAX, last);

        Self {
            key_to_count,
            count_to_node,
        }
    }

    fn inc(&mut self, key: String) {
        match self.key_to_count.get(&key).copied() {
            Some(count) => {
                self.key_to_count.insert(key.clone(), count + 1);
                let prev = self.count_to_node.get_mut(&count).unwrap();
                assert!(prev.here.remove(&key));
            }
            None => {
                self.key_to_count.insert(key.clone(), 1);
                match self.count_to_node.get_mut(&1) {
                    Some(node) => assert!(!node.here.insert(key)),
                    None => {
                        let prev = self.count_to_node.get(&0).unwrap();
                        let new_node = ListNode {
                            prev: 0,
                            next: prev.next,
                            num: 1,
                            here: HashSet::from([key]),
                        };

                        let prev = self.count_to_node.get_mut(&0).unwrap();
                        prev.next = 1;
                        let next = prev.next;
                        let next = self.count_to_node.get_mut(&next).unwrap();
                        next.prev = 1;
                    }
                }
            }
        }
    }

    fn dec(&self, key: String) {}

    fn get_max_key(&self) -> String {
        match self.count_to_node.get(&usize::MAX).unwrap().prev {
            0 => "".to_string(),
            other => self
                .count_to_node
                .get(&other)
                .unwrap()
                .here
                .iter()
                .next()
                .unwrap()
                .clone(),
        }
    }

    fn get_min_key(&self) -> String {
        match self.count_to_node.get(&0).unwrap().next {
            usize::MAX => "".to_string(),
            other => self
                .count_to_node
                .get(&other)
                .unwrap()
                .here
                .iter()
                .next()
                .unwrap()
                .clone(),
        }
    }
}

// * Your AllOne object will be instantiated and called as such:
//  * let obj = AllOne::new();
//  * obj.inc(key);
//  * obj.dec(key);
//  * let ret_3: String = obj.get_max_key();
//  * let ret_4: String = obj.get_min_key();
