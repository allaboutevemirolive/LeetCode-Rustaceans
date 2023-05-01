// https://leetcode.com/problems/min-stack/solutions/1977770/rust-single-stack-solution/
struct MinStack {
 q: Vec<(i32, i32)>,
 min: i32,
}

impl MinStack {
 fn new() -> Self {
  MinStack {
   q: vec![],
   min: i32::MAX,
  }
 }

 fn push(&mut self, val: i32) {
  if val < self.min {
   self.min = val;
  };
  self.q.push((val, self.min));
 }

 fn pop(&mut self) {
  self.q.pop();
  self.min = self.q.last().unwrap_or(&(-1, i32::MAX)).1
 }

 fn top(&self) -> i32 {
  self.q.last().unwrap().0
 }

 fn get_min(&self) -> i32 {
  self.min
 }
}
