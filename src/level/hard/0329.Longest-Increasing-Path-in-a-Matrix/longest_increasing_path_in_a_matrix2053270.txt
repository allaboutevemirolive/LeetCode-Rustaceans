// https://leetcode.com/problems/longest-increasing-path-in-a-matrix/solutions/2053270/rust-recursive-dfs-memoization/
impl Solution {
    fn dfs(matrix: &Vec<Vec<i32>>, visited: &mut Vec<Vec<Option<i32>>>, row: usize, col: usize) -> i32 {
        if let Some(length) = &visited[row][col] {
            return *length;
        }

        let m = matrix.len();
        let n = matrix[0].len();
        let v = matrix[row][col];

        let mut max_length = 0;
        for w in [0, 1, 0, !0, 0].windows(2) {
            let r = row.wrapping_add(w[0]);
            let c = col.wrapping_add(w[1]);
            if r < m && c < n {
                if matrix[r][c] > v {
                    max_length = max_length.max(Self::dfs(matrix, visited, r, c));
                }
            }
        }
        visited[row][col] = Some(max_length + 1);
        max_length + 1
    }

    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut visited = vec![vec![None; n]; m];
        let mut max_length = 1;
        for i in 0..m {
            for j in 0..n {
                max_length = max_length.max(Self::dfs(&matrix, &mut visited, i, j));
            }
        }
        max_length
    }
}