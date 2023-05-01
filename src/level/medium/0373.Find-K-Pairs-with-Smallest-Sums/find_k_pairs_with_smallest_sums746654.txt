// https://leetcode.com/problems/find-k-pairs-with-smallest-sums/solutions/746654/rust-0ms-simple-solution/
use std::i32::MAX;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut sums_pair:Vec<Vec<i32>> = Vec::new();
        let (len1, len2) = (nums1.len(), nums2.len());
        
        if len1 == 0 || len2 == 0 {
            return sums_pair;
        }
        
        let mut next = vec![0; len1];
        
        let mut floor = 0;
        let mut ceiling = 0;
        let mut curr = 0;
        
        for n in (0..k){
            sums_pair.push(vec!(nums1[curr], nums2[next[curr]]));
            next[curr] += 1;
            if next[curr] >= len2 {
                floor = curr +1;
                if floor >= len1 {
                    break;
                }
            }
            if curr == ceiling && ceiling +1 < len1{
                ceiling += 1;
            }
            let mut sum = MAX;
            for c in (floor..ceiling+1){
                if nums1[c] + nums2[next[c]] < sum{
                    sum = nums1[c] + nums2[next[c]];
                    curr = c;
                }
            }
        }
        
        return sums_pair;
    }
}