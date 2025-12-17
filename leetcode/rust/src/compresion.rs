struct Solution {}
impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut count = 0;
        let mut len = 0;
        for i in 0..chars.len() {
            count += 1;
            if chars.get(i) == chars.get(i + 1) {
            } else {
                chars[len] = chars[i];
                len += 1;
                if count != 1 {
                    for char in count.to_string().chars() {
                        chars[len] = char;
                        len += 1;
                    }
                }
                count = 0;
            }
        }
        chars.truncate(len);
        chars.len() as i32
    }
}
