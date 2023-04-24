// https://leetcode.com/problems/critical-connections-in-a-network/solutions/3306283/rust-tarjan-s-algorithm-with-hashset-slower-but-low-memory-usage/
use std::collections::HashSet;

impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut graph = vec![Vec::new(); n];
        let mut visited = vec![false; n];
        let mut disc = vec![0; n];
        let mut low = vec![0; n];
        let mut time = 0;
        let mut result = Vec::new();

        for connection in connections {
            let u = connection[0] as usize;
            let v = connection[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }

        for i in 0..n {
            if !visited[i] {
                Self::dfs(i, None, &mut time, &mut visited, &mut disc, &mut low, &graph, &mut result);
            }
        }

        result
    }

    fn dfs(
        u: usize,
        parent: Option<usize>,
        time: &mut i32,
        visited: &mut Vec<bool>,
        disc: &mut Vec<i32>,
        low: &mut Vec<i32>,
        graph: &Vec<Vec<usize>>,
        result: &mut Vec<Vec<i32>>,
    ) {
        visited[u] = true;
        *time += 1;
        disc[u] = *time;
        low[u] = *time;

        for &v in &graph[u] {
            if Some(v) != parent {
                if !visited[v] {
                    Self::dfs(v, Some(u), time, visited, disc, low, graph, result);
                    low[u] = low[u].min(low[v]);
                    if low[v] > disc[u] {
                        result.push(vec![u as i32, v as i32]);
                    }
                } else {
                    low[u] = low[u].min(disc[v]);
                }
            }
        }
    }
}