// https://leetcode.com/problems/cat-and-mouse/solutions/843054/rust-translated-52ms-100/
const DRAW: i32 = 0;
const MOUSE: i32 = 1;
const CAT: i32 = 2;

impl Solution {
    pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;

        fn parents(graph: &Vec<Vec<i32>>, m: i32, c: i32, t: i32) -> Vec<Vec<i32>> {
            let mut ans = vec![];
            if t == 2 {
                for &m2 in &graph[m as usize] {
                    ans.push(vec![m2, c, 3 - t]);
                }
            } else {
                for &c2 in &graph[c as usize] {
                    if c2 > 0 {
                        ans.push(vec![m, c2, 3 - t]);
                    }
                }
            }
            ans
        }

        let n = graph.len();

        let mut color = vec![vec![vec![0; 3]; 50]; 50];
        let mut degree = vec![vec![vec![0; 3]; 50]; 50];

        for m in 0..n {
            for c in 0..n {
                degree[m][c][1] = graph[m].len();
                degree[m][c][2] = graph[c].len();
                for &x in &graph[c] {
                    if x == 0 {
                        degree[m][c][2] -= 1;
                        break;
                    }
                }
            }
        }

        let mut queue = VecDeque::<Vec<i32>>::new();
        for i in 0..n {
            for t in 1..3 {
                color[0][i][t] = MOUSE;
                queue.push_back(vec![0, i as i32, t as i32, MOUSE]);
                if i > 0 {
                    color[i][i][t] = CAT;
                    queue.push_back(vec![i as i32, i as i32, t as i32, CAT]);
                }
            }
        }

        while !queue.is_empty() {
            // for nodes that are colored :
            let node = queue.pop_front().unwrap();
            let i = node[0];
            let j = node[1];
            let t = node[2];
            let c = node[3];
            // for every parent of this node i, j, t :
            for parent in parents(&graph, i, j, t) {
                let i2 = parent[0];
                let j2 = parent[1];
                let t2 = parent[2];
                // if this parent is not colored :
                if color[i2 as usize][j2 as usize][t2 as usize] == DRAW {
                    // if the parent can make a winning move (ie. mouse to MOUSE), do so
                    if t2 == c {
                        color[i2 as usize][j2 as usize][t2 as usize] = c;
                        queue.push_back(vec![i2, j2, t2, c]);
                    } else {
                        // else, this parent has degree[parent]--, and enqueue
                        // if all children of this parent are colored as losing moves
                        degree[i2 as usize][j2 as usize][t2 as usize] -= 1;
                        if degree[i2 as usize][j2 as usize][t2 as usize] == 0 {
                            color[i2 as usize][j2 as usize][t2 as usize] = 3 - t2;
                            queue.push_back(vec![i2, j2, t2, 3 - t2]);
                        }
                    }
                }
            }
        }
//        println!("{:?}", color);
        color[1][2][1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cat_mouse_game() {
        assert_eq!(
            Solution::cat_mouse_game(vec![
                vec![2, 5],
                vec![3],
                vec![0, 4, 5],
                vec![1, 4, 5],
                vec![2, 3],
                vec![0, 2, 3]
            ]),
            0
        );
    }
}