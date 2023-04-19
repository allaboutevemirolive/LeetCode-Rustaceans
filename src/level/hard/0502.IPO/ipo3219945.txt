// https://leetcode.com/problems/ipo/solutions/3219945/rust-two-priority-queues/
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let (mut pq1, mut pq2) = (BinaryHeap::new(), BinaryHeap::new());
        let (mut count, mut w) = (0, w);
        
        for i in 0 .. capital.len() { pq1.push(Reverse((capital[i], i))); }

        while pq1.is_empty() == false {
            while let Some(Reverse((c, i))) = pq1.peek() {
                if *c > w { break }
                
                pq2.push(profits[*i]);
                pq1.pop();
            }

            if let Some(p) = pq2.pop() {
                w += p;
                count += 1;
                if count == k { break }
            } else { break }
        }

        while let Some(p) = pq2.pop() {
            if count == k { break }
            count += 1;
            w += p; 
        }
        
        w
    }
}