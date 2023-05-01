// https://leetcode.com/problems/min-stack/solutions/2805440/rust-mono-descending-stack-o-n/
struct MinStack {
    desc_stack: Vec<i32>,
    stack: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        MinStack {
            desc_stack: vec![],
            stack: vec![],
        }
    }
    
    fn push(&mut self, val: i32) {
        self.stack.push(val);
        if let Some(&tail) = self.desc_stack.last() {
            if val <= tail {
                self.desc_stack.push(val);
            }
        } else {
            self.desc_stack.push(val);
        }
    }
    
    fn pop(&mut self) {
        let mut val = 0;
        if let Some(v) = self.stack.pop() {
            val = v;
        }
        if let Some(&tail) = self.desc_stack.last() {
            if val <= tail {
                self.desc_stack.pop();
            }
        }
    }
    
    fn top(&self) -> i32 {
        let mut ans = 0;
        if let Some(&tail) = self.stack.last() {
            ans = tail;
        }
        ans
    }
    
    fn get_min(&self) -> i32 {
        let mut ans = 0;
        if let Some(&tail) = self.desc_stack.last() {
            ans = tail;
        }
        ans
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */