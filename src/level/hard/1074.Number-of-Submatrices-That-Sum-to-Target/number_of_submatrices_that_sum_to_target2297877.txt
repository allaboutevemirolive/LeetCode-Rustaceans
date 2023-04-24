// https://leetcode.com/problems/number-of-submatrices-that-sum-to-target/solutions/2297877/rust-obvious-quartic-algorithm-slow-but-viable-for-small-inputs/
use std::collections::HashMap;

impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let prefixes = prefix_sum(&matrix);
        let height = matrix.len();
        let width = matrix[0].len();
        let mut acc = 0;
        for ru in 0 .. height {  // ru == row, upper
            for rl in ru .. height {  // rl == row, lower
                for cl in 0 .. width {  // column, left
                    for cr in cl .. width {  // column, right
                        let mut area = prefixes[rl][cr];
                        if cl > 0 { area -= prefixes[rl][cl-1]; }
                        if ru > 0 { area -= prefixes[ru-1][cr]; }
                        if cl > 0 && ru > 0 {
                            area += prefixes[ru-1][cl-1];
                        }
                        if area == target {
                            acc += 1;
                        }
                    }
                }
            }
        }
        acc
    }
}

fn prefix_sum(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let height = matrix.len();
    let width = matrix[0].len();
    let mut out: Vec<Vec<i32>> = vec![ vec![ 0; width ]; height ];
    out[0][0] = matrix[0][0].clone();
    for y in 1 .. height {
        out[y][0] = matrix[y][0] + out[y-1][0];
    }
    for x in 1 .. width {
        out[0][x] = matrix[0][x] + out[0][x-1];
    }
    for y in 1 .. height {
        for x in 1 .. width {
            out[y][x] = matrix[y][x] + out[y-1][x] + out[y][x-1] - out[y-1][x-1];
        }
    }
    out
}