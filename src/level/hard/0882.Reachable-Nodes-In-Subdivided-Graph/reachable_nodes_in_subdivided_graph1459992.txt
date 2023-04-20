// https://leetcode.com/problems/reachable-nodes-in-subdivided-graph/solutions/1459992/rust-dijkstra/
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
        let n = n as usize;
        let max_moves = max_moves as usize;

        let mut g = vec![vec![]; n];
        for (a, b, w) in edges
            .iter()
            .map(|e| (e[0] as usize, e[1] as usize, e[2] as usize))
        {
            g[a].push((b, w));
            g[b].push((a, w));
        }

        let mut d = vec![usize::MAX; n];
        let mut pq = BinaryHeap::new();
        d[0] = 0;
        pq.push(Reverse((0, 0)));

        while let Some(Reverse((wv, v))) = pq.pop() {
            if wv != d[v] {
                continue;
            }
            for &(u, wvu) in &g[v] {
                let nwu = wv + wvu + 1;
                if nwu < d[u] {
                    d[u] = nwu;
                    pq.push(Reverse((nwu, u)));
                }
            }
        }
        
        (edges
            .into_iter()
            .map(|e| (e[0] as usize, e[1] as usize, e[2] as usize))
            .map(|(a, b, c)| c.min(max_moves.saturating_sub(d[a]) + max_moves.saturating_sub(d[b])))
            .sum::<usize>()
            + d.into_iter().filter(|&x| x <= max_moves).count()) as _
    }
}