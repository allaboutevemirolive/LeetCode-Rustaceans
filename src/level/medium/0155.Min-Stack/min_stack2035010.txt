// https://leetcode.com/problems/min-stack/solutions/2035010/rust-solution/
struct MinStack {
    v: Vec<Vec<i32>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        return Self {
            v: Vec::new()
        }
    }
    
    fn push(&mut self, val: i32) {
        let n = self.v.len();
        if n == 0 {
            self.v.push(vec![val, val]);
            return;
        }
        
        
        
        let current_min = self.v[n-1][1];
        self.v.push(vec![val, std::cmp::min(current_min, val)]);
    }
    
    fn pop(&mut self) {
        self.v.pop();
    }
    
    fn top(&self) -> i32 {
        let n = self.v.len();
        return self.v[n-1][0];
    }
    
    fn get_min(&self) -> i32 {
        let n = self.v.len();
        return self.v[n-1][1];
    }
}