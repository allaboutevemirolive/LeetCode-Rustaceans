// https://leetcode.com/problems/min-stack/solutions/1795042/rust-62-ms-5-9-mb/
use std::collections::VecDeque;
struct MinStack {
    inner: VecDeque<i32>, 
}
impl MinStack {

    fn new() -> Self {
        Self { inner: VecDeque::new() }
    }
    fn push(&mut self, val: i32) {
        self.inner.push_front(val)
    }
    fn pop(&mut self) {
        self.inner.pop_front().unwrap();
    }
    fn top(&mut self) -> i32 {
        *self.inner.front().unwrap()
    }
    fn get_min(&mut self) -> i32 {
        let mut res: Vec<&i32> = self
			.inner
			.iter().collect();
        **res.iter().min().unwrap()
    }
}
