// https://leetcode.com/problems/longest-increasing-path-in-a-matrix/solutions/985953/rust-solution-bottom-up-dp/
let n = matrix.len();
let m = matrix.first().map(|v| v.len()).unwrap_or(0);