// https://leetcode.com/problems/ipo/solutions/3220110/rust-greedy/
impl Solution {
    pub fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut idx: Vec<usize> = (0..capital.len()).collect();
        idx.sort_by_key(|&i| capital[i]);

        let mut i = 0;
        let mut heap = std::collections::BinaryHeap::new();

        for _ in 0..k {
            while i < capital.len() && w >= capital[idx[i]] {
                heap.push(profits[idx[i]]);
                i += 1;
            }

            if let Some(t) = heap.pop() {
                w += t;
            } else {
                break;
            }
        }
        w
    }
}