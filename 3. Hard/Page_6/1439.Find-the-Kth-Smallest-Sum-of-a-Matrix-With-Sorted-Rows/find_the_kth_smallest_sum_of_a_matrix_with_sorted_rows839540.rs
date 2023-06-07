// https://leetcode.com/problems/find-the-kth-smallest-sum-of-a-matrix-with-sorted-rows/solutions/839540/rust-translated-4ms-100/
impl Solution {
    pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        fn count_le(
            mat: &Vec<Vec<i32>>,
            m: usize,
            n: usize,
            target: i32,
            r: usize,
            sum: i32,
            k: i32,
        ) -> i32 {
            if sum > target {
                return 0;
            }
            if r == m {
                return 1;
            }
            let mut ans = 0;
            for c in 0..n {
                let cnt = count_le(mat, m, n, target, r + 1, sum + mat[r][c], k - ans);
                if cnt == 0 {
                    break;
                }
                ans += cnt;
                if ans > k {
                    break;
                } // prune when count > k
            }
            ans
        }
        let m = mat.len();
        let n = mat[0].len();
        let mut left = m;
        let mut right = m * 5000;
        let mut ans = 0;
        while left <= right {
            let mid = left + (right - left) / 2;
            let cnt = count_le(&mat, m, n, mid as i32, 0, 0, k);
            if cnt >= k {
                ans = mid;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        ans as i32
    }
    //     use std::collections::BinaryHeap;
    //
    //     let m = mat.len();
    //     let n = mat[0].len();
    //     let mut h1 = BinaryHeap::<i32>::new();
    //     h1.push(0);
    //
    //     for i in 0..m {
    //         let mut h2 = BinaryHeap::<i32>::new();
    //         for &sum in h1.iter() {
    //             for j in 0..n {
    //                 h2.push(sum + mat[i][j]);
    //                 if h2.len() > k as usize {
    //                     h2.pop();
    //                 }
    //             }
    //         }
    //         h1 = h2;
    //     }
    //     h1.pop().unwrap_or(0)
    //
}