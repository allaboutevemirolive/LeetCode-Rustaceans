// https://leetcode.com/problems/longest-consecutive-sequence/solutions/2243602/rust-w-custom-set/
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut set   = MySet::new(1024);
        let mut max_c = 0;
        
        nums.into_iter().for_each(|n| { set.insert(n); });
        
        while let Some(n) = set.pop() {
            let mut m = n + 1;
            let mut c = 1;
            
            while set.remove(&m) {
                c += 1;
                m += 1;
            }
            m = n - 1;
            
            while set.remove(&m) {
                c += 1;
                m -= 1;
            }
            max_c = max_c.max(c);
        }
        max_c
    }
}

struct MySet {
    buckets : Vec<Option<Vec<i32>>>,
    next    : usize,
}
impl MySet {
    fn new(size: usize) -> Self {
        Self { buckets: vec![None; size], next: 0 }
    }
    fn insert(&mut self, key: i32) {
        let hash = key.abs() as usize % self.buckets.len();

        match &mut self.buckets[hash] {
            None => {
                self.buckets[hash] = Some(vec![key]);
            },
            Some(bucket) => {
                match bucket.binary_search(&key) {
                    Err(i) => { bucket.insert(i, key); },
                    Ok(_)  => { },
                }
            }
        }
    }
    fn remove(&mut self, key: &i32) -> bool {
        let hash = key.abs() as usize % self.buckets.len();
        
        match &mut self.buckets[hash] {
            None => { false },
            Some(bucket) => {
                match bucket.binary_search(&key) {
                    Ok(i)  => { bucket.remove(i); true },
                    Err(_) => { false },
                }
            }
        }
    }
    fn pop(&mut self) -> Option<i32> {
        while self.next < self.buckets.len() {
            if let Some(v) = &mut self.buckets[self.next] {
                if v.len() > 0 {
                    return v.pop();
                }
            }
            self.next += 1;
        }
        None
    }
}