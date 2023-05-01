// https://leetcode.com/problems/course-schedule-ii/solutions/3317425/rust-2-approaches/
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        fn khan_topological_sort(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
            use std::collections::*;
            let mut adj = vec![vec![]; num_courses as usize];
            let mut in_degree = vec![0; num_courses as usize];
            for to_from in prerequisites {
                let from = to_from[1] as usize;
                let to = to_from[0] as usize;
                adj[from].push(to);
                in_degree[to] += 1;
            }
            let mut queue = VecDeque::new();
            for i in 0..num_courses as usize {
                if in_degree[i] == 0 {
                    queue.push_back(i);
                }
            }
            let mut ans = vec![];
            while let Some(v) = queue.pop_front() {
                ans.push(v as i32);
                for &u in &adj[v] {
                    in_degree[u] -= 1;
                    if in_degree[u] == 0 {
                        queue.push_back(u);
                    }
                }
            }
            if ans.len() != num_courses as usize {
                return vec![];
            }
            ans
        }
        fn dfs_topological_sort(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
            fn dfs(v: usize, adj: &[Vec<usize>], visited: &mut [i32], ans: &mut Vec<i32>) -> bool {
                //return has_cycle
                if visited[v] == 1 {
                    return true;
                }
                visited[v] = 1;
                for &u in &adj[v] {
                    if visited[u] != 2 && dfs(u, adj, visited, ans) {
                        return true;
                    }
                }
                ans.push(v as i32);
                visited[v] = 2;
                false
            }
            let mut adj = vec![vec![]; num_courses as usize];
            for to_from in prerequisites {
                let from = to_from[1] as usize;
                let to = to_from[0] as usize;
                adj[from].push(to);
            }
            let mut visited = vec![0; num_courses as usize];
            let mut ans = vec![];
            for v in 0..num_courses as usize {
                if visited[v] == 0 && dfs(v, &adj, &mut visited, &mut ans) {
                    return vec![];
                }
            }
            ans.reverse();
            ans
        }
        dfs_topological_sort(num_courses, prerequisites)    
    }
}