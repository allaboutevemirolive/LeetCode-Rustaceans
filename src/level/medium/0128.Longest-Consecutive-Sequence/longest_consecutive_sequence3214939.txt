// https://leetcode.com/problems/longest-consecutive-sequence/solutions/3214939/rust-dsu-solution/
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::*;
        if nums.is_empty() {
            return 0;
        }
        let mut idx_map = HashMap::new();
        let mut uf_size = vec![0; nums.len()];
        let mut uf_array = vec![0; nums.len()];
        for (idx, num) in nums.into_iter().enumerate() {
            uf_array[idx] = idx;
            uf_size[idx] = 1;
            idx_map.insert(num, idx);
        }
        fn find(uf_array: &mut Vec<usize>, x: usize) -> usize {
            if uf_array[x] == x {
                return x;
            }
            uf_array[x] = find(uf_array, uf_array[x]);
            uf_array[x]
        }
        fn union(uf_array: &mut Vec<usize>, uf_size: &mut [i32], x: usize, y: usize) {
            let mut x = find(uf_array, x);
            let mut y = find(uf_array, y);
            if uf_size[x] > uf_size[y] {
                std::mem::swap(&mut x, &mut y);
            }
            uf_array[x] = y;
            uf_size[y] += uf_size[x];
        }
        for (&k, &a) in &idx_map {
            if let Some(&b) = idx_map.get(&(k - 1)) {
                union(&mut uf_array, &mut uf_size, a, b);
            }
        }
        uf_size.into_iter().max().unwrap()
    }
}