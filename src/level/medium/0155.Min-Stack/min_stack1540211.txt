// https://leetcode.com/problems/min-stack/solutions/1540211/rust-simple-solution-using-a-vec/
struct MinStack {
    stack: Vec<(i32, i32)>,
}

impl MinStack {
    fn new() -> Self {
        MinStack { stack: Vec::new() }
    }

    fn push(&mut self, val: i32) {
        let mut min = val;

        if let Some(&(_, m)) = self.stack.last() {
            min = min.min(m);
        }

        self.stack.push((val, min));
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack.last().copied().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.stack.last().copied().unwrap().1
    }
}