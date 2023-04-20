// https://leetcode.com/problems/my-calendar-iii/solutions/2673392/rust/
use std::collections::HashMap;

struct MyCalendarThree {
    b: HashMap<i32, i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {

    fn new() -> Self {
        MyCalendarThree { b: HashMap::new() }
    }
    
    fn book(&mut self, start: i32, end: i32) -> i32 {
        *(self.b.entry(start).or_insert(0)) += 1;
        *(self.b.entry(end).or_insert(0)) -= 1;
        
        Self::sorted(self.b.iter().collect::<Vec<(&i32, &i32)>>()).into_iter().fold((0, 0), |(mut prev, mut max), (_, v)| {
            prev += *v;
            (prev, std::cmp::max(max, prev))
        }).1
    }
    
    fn sorted<'a>(mut v: Vec<(&'a i32, &'a i32)>) -> Vec<(&'a i32, &'a i32)> {
        v.sort_by(|a, b| a.0.cmp(b.0));
        v
    }
}