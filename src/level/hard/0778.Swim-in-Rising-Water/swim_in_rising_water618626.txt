// https://leetcode.com/problems/swim-in-rising-water/solutions/618626/rust-binaryheap-4ms/
impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let mut pq: BinaryHeap<(i32, i32, i32)> = BinaryHeap::new(); 
        let mut res = 0; 
        let n = grid.len(); 
        let mut cc:HashSet<(i32, i32)> = HashSet::new(); 
        let directions = [-1_i32, 0, 1, 0, -1];
        pq.push((-1 * grid[0][0], 0, 0)); 
        while true {
            let top = pq.pop().unwrap(); 
            res = min(res, top.0); 
           
            if top.1 == top.2 && top.2 == (n as i32 - 1) { return -1 * res }
            cc.insert((top.1, top.2));

            for d in (0..4) {
                let ni = top.1 + directions[d]; 
                let nj = top.2 + directions[d+1]; 
                if 0 <= ni && ni < n as i32 && 0 <= nj && nj < n as i32 && !cc.contains(&(ni, nj)){
                    pq.push((-1 * grid[ni as usize][nj as usize], ni, nj));
                }
            }
        }
        0
    }
}