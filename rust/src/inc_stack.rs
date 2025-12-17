use std::cmp::min;

struct CustomStack {
    stack: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {
    fn new(max_size: i32) -> Self {
        Self {
            stack: Vec::with_capacity(max_size as usize),
        }
    }

    fn push(&mut self, x: i32) {
        if self.stack.len() == self.stack.capacity() {
            return;
        }
        self.stack.push((x, 0));
    }

    fn pop(&mut self) -> i32 {
        let (item, incr) = match self.stack.pop() {
            Some(item) => item,
            None => return -1,
        };

        if let Some(item) = self.stack.last_mut() {
            item.1 += incr
        }

        item + incr
    }

    fn increment(&mut self, k: i32, val: i32) {
        if self.stack.is_empty() {
            return;
        }
        let index = min(self.stack.len(), k as usize) - 1;
        self.stack[index].1 += val;
    }
}

// /**
//  * Your CustomStack object will be instantiated and called as such:
//  * let obj = CustomStack::new(maxSize);
//  * obj.push(x);
//  * let ret_2: i32 = obj.pop();
//  * obj.increment(k, val);
//  */
