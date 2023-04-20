// https://leetcode.com/problems/kth-smallest-number-in-multiplication-table/solutions/1361953/rust-binary-search/
impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let k = k as usize;
        let mut low: i32 = 1;
        let mut high: i32 = m * n;

        while low < high {
            let mut count: usize = 0;
            let middle: i32 = (low + high) / 2;

            let mut j = n;
            for i in 0..m {
                while j > 0 && (i + 1) * j > middle {
                    j -= 1;
                }
                if j == 0 {
                    break;
                }
                count += j as usize;
            }

            if count < k {
                low = middle + 1;
            } else {
                high = middle;
            }
        }

        low
    }
}