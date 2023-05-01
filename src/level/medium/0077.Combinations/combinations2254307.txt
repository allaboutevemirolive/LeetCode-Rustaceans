// https://leetcode.com/problems/combinations/solutions/2254307/rust-iterator-4ms-100-2-8mb-73-44/
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut combo = Combo::new(n, k as usize);
        combo.collect()
    }
}

struct Combo {
    len: usize,
    v: Vec<i32>,
    maxi: Vec<i32>,
    finished: bool,
}

impl Combo {
    pub fn new(max: i32, len: usize) -> Self {
        let mut v = Vec::with_capacity(len);
        let mut maxi = Vec::with_capacity(len);
        for i in 1..=len {
            v.push(i as i32);
            maxi.push(max - (len - i) as i32);
        }        
        Self {
            len,
            v,
            maxi,
            finished: false,
        }
    }
}

impl Iterator for Combo {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let next = self.v.clone();
        
        let mut i = self.len - 1;        
        while i > 0 && self.v[i] >= self.maxi[i] {
            i -= 1;
        }
        
        if i == 0 && self.v[0] == self.maxi[0] {
            self.finished = true;
        } else {
            self.v[i] += 1;
            i += 1;
            while i < self.len {
                self.v[i] = self.v[i-1] + 1;
                i += 1;
            }
        }
        Some(next)
    }
}