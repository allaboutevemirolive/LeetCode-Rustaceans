// https://leetcode.com/problems/shortest-path-in-a-grid-with-obstacles-elimination/solutions/1485640/rust-0ms-bfs-search-with-manhattan-distance-optimization-with-a-bit-of-comments/
impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let row_cnt = grid.len();
        let column_cnt = grid[0].len();
        let k = k as usize;
        let mut prev = vec![(0, 0, k)];
        let mut result = 0;
        // record the shortest path length to fasten search
        let mut min_result = i32::MAX;
        let mut searched = vec![vec![vec![false; k + 1]; column_cnt]; row_cnt];
        while !prev.is_empty() {
            let mut next = vec![];
            for (row_index, column_index, k_left) in prev.into_iter() {
                // filter invalid search info
                if row_index == row_cnt || column_index == column_cnt {
                    continue;
                }
                // arrived at the end node, return the result
                if row_index + 1 == row_cnt && column_index + 1 == column_cnt {
                    return result;
                }
                // every k "level" is different, same position in different levels may not be both searched in a same path
                if searched[row_index][column_index][k_left] {
                    continue;
                }
                // shortest length from this node, manhattan distance
                let shortest_result = (result as usize + row_cnt - 1 - row_index + column_cnt - 1 - column_index) as i32;
                if k_left + row_index + column_index + 1 > row_cnt - 1 + column_cnt - 1 {
                    // will definite reach the end, prev+delta_x+delta_y, remember to count this node
                    min_result = min_result.min(shortest_result);
                    continue;
                }
                if shortest_result > min_result {
                    // no need to search further, we cannot improve the min_result from this node
                    continue;
                }
                // mark searched
                searched[row_index][column_index][k_left] = true;
                let val = grid[row_index][column_index];
                // search in every possible direction
                if val == 1 && k_left > 0 {
                    next.push((row_index + 1, column_index, k_left - 1));
                    next.push((row_index, column_index + 1, k_left - 1));
                    if row_index > 0 {
                        next.push((row_index - 1, column_index, k_left - 1));
                    }
                    if column_index > 0 {
                        next.push((row_index, column_index - 1, k_left - 1));
                    }
                }
                if val == 0 {
                    next.push((row_index + 1, column_index, k_left));
                    next.push((row_index, column_index + 1, k_left));
                    if row_index > 0 {
                        next.push((row_index - 1, column_index, k_left));
                    }
                    if column_index > 0 {
                        next.push((row_index, column_index - 1, k_left));
                    }
                }
            }
            prev = next;
            // try next level
            result += 1;
        }
        // we know the shortest path without reaching the end node
        if min_result != i32::MAX {
            return min_result;
        }
        // path not found at all
        -1
    }
}