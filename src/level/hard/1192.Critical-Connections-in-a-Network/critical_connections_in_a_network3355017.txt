// https://leetcode.com/problems/critical-connections-in-a-network/solutions/3355017/rust-implementation/
impl Solution {
fn dfs(
        adj: &Vec<Vec<i32>>,
        parent: i32,
        node: i32,
        visited: &mut Vec<bool>,
        timer: &mut i32,
        time_in: &mut Vec<i32>,
        low: &mut Vec<i32>,
        bridges: &mut Vec<Vec<i32>>,
    ) {
        visited[node as usize] = true;
        time_in[node as usize] = *timer;
        low[node as usize] = *timer;

        *timer += 1;

        for &neighbor in &adj[node as usize] {
            if parent == neighbor {
                continue;
            }
            let neighbor_index = neighbor as usize;

            if !visited[neighbor_index] {
                Self::dfs(adj, node, neighbor, visited, timer, time_in, low, bridges);

                low[node as usize] = low[neighbor_index].min(low[node as usize]);
                if low[neighbor_index] > time_in[node as usize] {
                    bridges.push(vec![node, neighbor]);
                }

            } else {
                low[node as usize] = low[neighbor_index].min(low[node as usize]);
            }
        }
    }

    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut adj: Vec<Vec<i32>> = vec![vec![]; n as usize];

        for connection in connections {
            let (u, v) = (connection[0] as i32, connection[1] as i32);
            adj[u as usize].push(v);
            adj[v as usize].push(u);
        }

        let mut tim_in: Vec<i32> = vec![0; n as usize];
        let mut low: Vec<i32> = vec![0; n as usize];
        let mut visited: Vec<bool> = vec![false; n as usize];
        let mut bridges: Vec<Vec<i32>> = Vec::new();
        let mut timer: i32 = 0;

        Self::dfs(
            &adj,
            -1,
            0,
            &mut visited,
            &mut timer,
            &mut tim_in,
            &mut low,
            &mut bridges,
        );

        bridges
    }
}