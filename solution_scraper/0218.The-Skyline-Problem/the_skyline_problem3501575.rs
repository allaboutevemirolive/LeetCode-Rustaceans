// https://leetcode.com/problems/the-skyline-problem/solutions/3501575/easy-to-implement-o-n-log-n-heap-solution-in-rust-beat-100-in-runtime/
impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = buildings.len();
        let mut ret = Vec::<_>::with_capacity(n);
        let mut heap = std::collections::BinaryHeap::<(i32, i32)>::with_capacity(n);
        let mut x = buildings[0][0];
        let mut i = 0;
        while i < n || !heap.is_empty() {
            while i < n && buildings[i][0] <= x {
                heap.push((buildings[i][2], buildings[i][1]));
                i += 1;
            }
            while !heap.is_empty() && heap.peek().unwrap().1 <= x {
                heap.pop();
            }
            let y = heap
                .peek()
                .map(|t| t.0)
                .unwrap_or(0);
            ret.push(vec![x, y]);
            x = *[heap.peek().map(|t| t.1), buildings.get(i).map(|b| b[0])]
                .iter()
                .flatten()
                .min()
                .unwrap_or(&x);
        }
        let mut h = Some(0);
        ret.retain(|a| h.replace(a[1]).unwrap() != a[1]);
        ret
    }
}