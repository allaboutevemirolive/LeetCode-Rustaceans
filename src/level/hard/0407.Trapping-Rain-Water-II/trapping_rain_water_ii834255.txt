// https://leetcode.com/problems/trapping-rain-water-ii/solutions/834255/rust-translated-4ms-100/
impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let m = height_map.len();
        if m == 0 {
            return 0;
        }
        let n = height_map[0].len();
        let mut heap: BinaryHeap<(Reverse<i32>, i32, i32)> = BinaryHeap::new();
        let mut visited = vec![vec![false; n]; m];
        let mut ans = 0;
        for c in 0..n {
            heap.push((Reverse(height_map[0][c]), 0, c as i32));
            heap.push((Reverse(height_map[m - 1][c]), m as i32 - 1, c as i32));
            visited[0][c] = true;
            visited[m - 1][c] = true;
        }
        for r in 1..m {
            heap.push((Reverse(height_map[r][0]), r as i32, 0));
            heap.push((Reverse(height_map[r][n - 1]), r as i32, n as i32 - 1));
            visited[r][0] = true;
            visited[r][n - 1] = true;
        }
        const DIRS: [i32; 5] = [0, -1, 0, 1, 0];
        while !heap.is_empty() {
            let min = heap.pop().unwrap();
            for i in 0..4 {
                let r = min.1 + DIRS[i];
                let c = min.2 + DIRS[i + 1];
                if r > 0
                    && c > 0
                    && r < m as i32
                    && c < n as i32
                    && !visited[r as usize][c as usize]
                {
                    heap.push((
                        Reverse(std::cmp::max((min.0).0, height_map[r as usize][c as usize])),
                        r,
                        c,
                    ));
                    visited[r as usize][c as usize] = true;
                    ans += std::cmp::max(0, (min.0).0 - height_map[r as usize][c as usize]);
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trap_rain_water() {
        assert_eq!(
            Solution::trap_rain_water(vec![
                vec![1, 4, 3, 1, 3, 2],
                vec![3, 2, 1, 3, 2, 4],
                vec![2, 3, 3, 2, 3, 1]
            ]),
            4
        )
    }
}