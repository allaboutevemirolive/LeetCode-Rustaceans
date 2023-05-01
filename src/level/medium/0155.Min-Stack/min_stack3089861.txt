// https://leetcode.com/problems/min-stack/solutions/3089861/rust-solution/
use std::collections::*;

struct MinStack {
  btreeset: BTreeSet<(i32,usize)>,
  indices: VecDeque<(i32, usize)>,
  n:usize
}

impl MinStack {
  fn new() -> Self {
    MinStack {
      btreeset: BTreeSet::new(),
      indices: VecDeque::new(),
      n:0
    }
  }
  
  fn push(&mut self, val: i32) {
    let ci = self.n;
    self.btreeset.insert((val, ci));
    self.indices.push_back((val, ci));
    self.n += 1;
  }
  
  fn pop(&mut self) {
    if let Some(v) = self.indices.pop_back() {
      self.btreeset.remove(&v);
    }
  }
  
  fn top(&self) -> i32 {
    if let Some((v,_)) = self.indices.get(self.indices.len()-1) {
      *v
    } else {
      0
    }
  }
  
  fn get_min(&self) -> i32 {
    if let Some((v,_)) = self.btreeset.iter().next() {
      *v
    } else {
      0
    } 
  }
}