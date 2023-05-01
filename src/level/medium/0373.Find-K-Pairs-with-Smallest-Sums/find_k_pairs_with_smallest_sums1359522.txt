// https://leetcode.com/problems/find-k-pairs-with-smallest-sums/solutions/1359522/rust-binary-heap/
use std::collections::BinaryHeap;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
	    // nums1 = [1,7,11], nums2 = [2,4,6], k = 3
        // 1+2  1+4  1+6
        // 7+2  7+4  7+6
        // 11+2 11+4 11+6
        let mut res: Vec<Vec<i32>> = Vec::new();
        let (n1, n2) = (nums1.len(), nums2.len());
        // start of with the first column
        let mut heap: BinaryHeap<(i32, usize, usize)> = BinaryHeap::from(
            (0..n1)
                .map(|x| (-nums1[x] - nums2[0], x, 0))
                .collect::<Vec<(i32, usize, usize)>>(),
        );
        
        // pop off the lowest one
        while let Some((_, i, j)) = heap.pop() {
            res.push(vec![nums1[i], nums2[j]]);
            // end if reach k
            if res.len() == k as usize {
                break;
            }
            // push the right handside sum into the heap
            if j < n2 - 1 {
                heap.push((-nums1[i] - nums2[j + 1], i, j + 1));
            }
        }

        res
    }
}