// https://leetcode.com/problems/find-k-pairs-with-smallest-sums/solutions/833649/rust-translated-0ms-100/
impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, mut k: i32) -> Vec<Vec<i32>> {
        use std::collections::BinaryHeap;

        let mut ans = vec![];
        if k == 0 || nums1.is_empty() || nums2.is_empty() {
            return ans;
        }
        let mut heap = BinaryHeap::<Vec<i32>>::new();
        for i in 0..std::cmp::min(k as usize, nums1.len()) {
            heap.push(vec![-(nums1[i]+nums2[0]), nums1[i], nums2[0], 0]);
        }
        while k > 0 && !heap.is_empty() {
//            println!("{:?}", heap);
            k -= 1;
            let cur = heap.pop().unwrap();
            ans.push(vec![cur[1], cur[2]]);
            if cur[3] == nums2.len() as i32 - 1 {
                continue;
            }
            heap.push(vec![-cur[1] - nums2[cur[3] as usize + 1], cur[1], nums2[cur[3] as usize + 1], cur[3] + 1]);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_smallest_pairs() {
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3),
            vec![vec![1, 2], vec![1, 4], vec![1, 6]]
        )
    }

    #[test]
    fn test_k_smallest_pairs_02() {
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 2),
            vec![vec![1, 1], vec![1, 1]]
        )
    }

    #[test]
    fn test_k_smallest_pairs_03() {
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 2], vec![3], 2),
            vec![vec![1, 3], vec![2, 3]]
        )
    }

    #[test]
    fn test_k_smallest_pairs_04() {
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 10),
            vec![
                vec![1, 1],
                vec![1, 1],
                vec![1, 2],
                vec![1, 2],
                vec![1, 3],
                vec![1, 3],
                vec![2, 1],
                vec![2, 2],
                vec![2, 3]
            ]
        )
    }

    #[test]
    fn test_k_smallest_pairs_05() {
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 2, 3, 4, 5, 6], vec![3, 5, 7, 9], 3),
            vec![vec![1, 3], vec![2, 3], vec![1, 5]]
        )
    }
}