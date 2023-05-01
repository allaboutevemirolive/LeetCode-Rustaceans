// https://leetcode.com/problems/course-schedule/solutions/3288620/rust-dfs-and-khan-algorithms/
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        fn dfs_topological_sort(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
            fn dfs(
                v: usize,
                adj: &Vec<Vec<usize>>,
                visited: &mut Vec<u8>,
                courses_order: &mut Vec<usize>,
            ) -> bool {
                if visited[v] == 1 {
                    return false;
                }
                visited[v] = 1;
                for &u in &adj[v] {
                    if visited[u] != 2 && !dfs(u, adj, visited, courses_order) {
                        return false;
                    }
                }
                visited[v] = 2;
                courses_order.push(v);
                true
            }
            let mut visited = vec![0; num_courses as usize];
            let mut adj = vec![vec![]; num_courses as usize];
            for p in prerequisites {
                let (from, to) = (p[0], p[1]);
                adj[from as usize].push(to as usize);
            }
            let mut courses_order = vec![];
            for course in 0..num_courses as usize {
                if visited[course] == 0 && !dfs(course, &adj, &mut visited, &mut courses_order) {
                    return false;
                }
            }
            // courses_order.reverse();
            // println!("{:?}", courses_order);
            true
        }

        fn khan_topological_sort(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
            use std::collections::*;
            let mut queue = VecDeque::new();
            let mut in_degrees = vec![0; num_courses as usize];
            let mut adj = vec![vec![]; num_courses as usize];

            for p in prerequisites {
                let (from, to) = (p[0], p[1]);
                in_degrees[to as usize] += 1;
                adj[from as usize].push(to as usize);
            }

            let mut courses_order = vec![];
            for (course, &degree) in in_degrees.iter().enumerate() {
                if degree == 0 {
                    queue.push_back(course);
                }
            }

            while let Some(course) = queue.pop_front() {
                courses_order.push(course);
                for &next_course in &adj[course] {
                    in_degrees[next_course] -= 1;
                    if in_degrees[next_course] == 0 {
                        queue.push_back(next_course);
                    }
                }
            }

            // println!("{:?}", courses_order);
            courses_order.len() == num_courses as usize
        }
        khan_topological_sort(num_courses, prerequisites)
    }
}