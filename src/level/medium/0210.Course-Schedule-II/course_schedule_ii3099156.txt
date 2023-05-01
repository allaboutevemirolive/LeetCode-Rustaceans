// https://leetcode.com/problems/course-schedule-ii/solutions/3099156/rust-simple-topological-sort/
impl Solution {
    pub fn find_order(n: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        // graph and indegree array
        let (graph, mut degree) =
            prerequisites
                .iter()
                .fold((vec![vec![]; n], vec![0; n]), |(mut g, mut d), edge| {
                    g[edge[1] as usize].push(edge[0]);
                    d[edge[0] as usize] += 1;
                    (g, d)
                });
        // q for bfs/dfs
        let mut q = std::collections::VecDeque::new();
        // start with course without any prereqs - inedgree = 0
        (0..n)
            .filter(|&i| degree[i] == 0)
            .for_each(|i| q.push_back(i as i32));
        let mut order = Vec::new();
        // BFS here,any of `pop_front` or `pop_back` should be able
        // to finish
        while let Some(course) = q.pop_front() {
            order.push(course);
            graph[course as usize].iter().for_each(|&next| {
                degree[next as usize] -= 1;
                if degree[next as usize] == 0 {
                    q.push_back(next);
                }
            });
        }
        if order.len() != n {
            return vec![];
        }
        order
    }
}