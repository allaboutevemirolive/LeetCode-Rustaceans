// https://leetcode.com/problems/min-stack/solutions/2781575/rust-solution/
struct MinStack {
    stack: Vec<(i32, i32)>  // (val, min)
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        Self { stack: Vec::new() }
    }
    
    fn push(&mut self, val: i32) {
        let min = if let Some(value) = self.stack.last() {
            value.1.min(val)
        } else {
            val
        };
        self.stack.push((val, min));
    }
    
    fn pop(&mut self) {
        self.stack.pop();
    }
    
    fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }
    
    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}