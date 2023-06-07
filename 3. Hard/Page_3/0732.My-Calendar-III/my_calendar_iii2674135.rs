// https://leetcode.com/problems/my-calendar-iii/solutions/2674135/rust-7ms-vector-binary-search-concise-solution/
struct MyCalendarThree {
    v: Vec<(i32, i32)>,
    m: i32,
}

impl MyCalendarThree {

    fn new() -> Self {
        Self{v: vec![], m: 0}
    }
    
    fn book(&mut self, start: i32, end: i32) -> i32 {
        let mut i =
        match self.v.binary_search_by(|probe| probe.0.cmp(&start)) {
            Ok(idx) => idx,
            Err(idx) => {
                self.v.insert(idx, (start, if idx > 0 {self.v[idx - 1].1} else {0}));
                idx
            }
        };
        
        let vlen = self.v.len();
        let mut lastk = 0;
        while i < vlen && self.v[i].0 < end {
            self.v[i].1 += 1;
            lastk = self.v[i].1;
            self.m = self.m.max(lastk);
            i += 1;
        }
        if i == vlen || self.v[i].0 != end {
            self.v.insert(i, (end, lastk - 1));
        }
        self.m
    }
}