// https://leetcode.com/problems/longest-increasing-path-in-a-matrix/solutions/705129/rust-topological-sort-bfs-branches-cut-longer-time-code-than-dp-32ms/
use std::collections::VecDeque;
use std::iter::FromIterator;

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let h = matrix.len();
        if h == 0 { return 0; }
        let w = matrix[0].len();
        let dir = [[1, 0], [-1, 0], [0, 1], [0, -1]];
        // Create Graph
        let mut graph = &mut vec![(0, vec![]); w * h];// indegree,neighbor

        for r in 0..h {
            for c in 0..w {
                for d in &dir {
                    let (nr, nc) = (r as i32 + d[0], c as i32 + d[1]);
                    if nr < 0 || nc < 0 { continue; }
                    let (nr, nc) = (nr as usize, nc as usize);
                    if nr >= h || nc >= w { continue; }
                    if matrix[nr][nc] > matrix[r][c] {
                        let neibghbor_index = nr * w + nc;
                        graph[r * w + c].1.push(neibghbor_index);
                        graph[neibghbor_index].0 += 1;
                    }
                }
            }
        }

        let mut q: VecDeque<usize> = VecDeque::from_iter(graph.iter().enumerate().filter_map(|(i, (in_deg, _))| if *in_deg == 0 { Some(i) } else { None }));

        let ans = &mut 0;
        while !q.is_empty() {
            for _ in (0..q.len()) {
                let x = q.pop_front().unwrap();
                for neibghbor in graph[x].1.clone() {
				   // THIS IS IMPORTANT
                    graph[neibghbor].0 -= 1;
                    if graph[neibghbor].0 == 0 {
                        q.push_back(neibghbor);
                    }
                }
            }
            *ans += 1;
        }

        *ans
    }
}