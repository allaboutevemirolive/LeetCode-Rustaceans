// https://leetcode.com/problems/maximum-frequency-stack/solutions/1087576/rust-solution/
use std::collections::HashMap;

struct FreqStack {
    frequency_map: HashMap<i32, usize>,
    frequency_order: Vec<Vec<i32>>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {
    fn new() -> Self {
        FreqStack {
            frequency_map: HashMap::new(),
            frequency_order: Vec::new(),
        }
    }
    
    fn push(&mut self, x: i32) {
        let freq = self.frequency_map.entry(x).or_insert(0);
        
        if *freq >= self.frequency_order.len() {
            self.frequency_order.push(Vec::new());
        }
        self.frequency_order[*freq].push(x);
        
        *freq += 1;
    }
    
    fn pop(&mut self) -> i32 {
        let mut order_last = self.frequency_order.last_mut().unwrap();
        
        let val = order_last.pop().unwrap();
        
        if order_last.is_empty() {
            self.frequency_order.pop();
        }
        
        *self.frequency_map.get_mut(&val).unwrap() -= 1;
        
        val
    }
}

/**
 * Your FreqStack object will be instantiated and called as such:
 * let obj = FreqStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 */