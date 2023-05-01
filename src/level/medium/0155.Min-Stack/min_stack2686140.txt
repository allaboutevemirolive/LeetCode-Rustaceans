// https://leetcode.com/problems/min-stack/solutions/2686140/rust-monotonic-stack/
struct MinStack {
    data: Vec<i32>,
    min: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            data: vec![],
            min: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        self.data.push(val);
        if self.min.is_empty() || val <= *self.min.last().unwrap() {
            self.min.push(val);
        }
    }

    fn pop(&mut self) {
        if let Some(popped) = self.data.pop() {
            if *self.min.last().unwrap() == popped {
                self.min.pop();
            }
        }
    }

    fn top(&self) -> i32 {
        *self.data.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min.last().unwrap()
    }
}