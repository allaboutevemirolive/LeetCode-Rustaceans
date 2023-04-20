// https://leetcode.com/problems/remove-boxes/solutions/1568489/rust-top-down-dp/
impl Solution {
    pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
        let n = boxes.len();
        let mut memo = vec![vec![vec![0i32; n]; n]; n];

        fn dp(
            i: usize,
            j: usize,
            k: usize,
            boxes: &Vec<i32>,
            memo: &mut Vec<Vec<Vec<i32>>>,
        ) -> i32 {
            if i > j {
                return 0;
            }
            if memo[i][j][k] > 0 {
                return memo[i][j][k];
            }
            let mut res;
            {
                let (mut i, mut k) = (i, k);
                while i + 1 <= j && boxes[i + 1] == boxes[i] {
                    i += 1;
                    k += 1;
                }
                res = ((k + 1) * (k + 1)) as i32 + dp(i + 1, j, 0, boxes, memo);

                for m in (i + 1)..=j {
                    if boxes[i] == boxes[m] {
                        res = res
                            .max(dp(i + 1, m - 1, 0, boxes, memo) + dp(m, j, k + 1, boxes, memo));
                    }
                }
            }
            memo[i][j][k] = res;
            return res;
        }
        return dp(0, n - 1, 0, &boxes, &mut memo);
    }
}