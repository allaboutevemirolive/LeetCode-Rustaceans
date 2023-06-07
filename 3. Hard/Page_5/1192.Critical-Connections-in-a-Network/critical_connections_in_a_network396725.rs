// https://leetcode.com/problems/critical-connections-in-a-network/solutions/396725/rust-tarjan/
use std::collections::HashMap;
type GraphUnweightedAL = HashMap<usize,Vec<usize>>;

fn make_graph(n: usize, conn: &Vec<Vec<usize>>) -> GraphUnweightedAL {
    let mut mp = HashMap::new();
    (0..n).for_each(|i| {
        mp.insert(i, vec![]);
    });
    conn.into_iter().for_each(|edge| {
        let (u, v) = (edge[0], edge[1]);
        (*mp.get_mut(&u).unwrap()).push(v.clone());
        (*mp.get_mut(&v).unwrap()).push(u.clone());
    });
    mp
}

fn find_bridges_util(u: &usize, g: &GraphUnweightedAL, parent: &mut Vec<Option<usize>>, vis: &mut Vec<bool>, dfn: &mut Vec<usize>, low: &mut Vec<usize>, time: &mut usize, result: &mut Vec<Vec<usize>>) {
    vis[*u] = true;
    *time += 1;
    dfn[*u] = *time;
    low[*u] = *time;
    for v in g.get(u).unwrap() {
        if !vis[*v] {
            parent[*v] = Some(*u);
            find_bridges_util(v, g, parent, vis, dfn, low, time, result);
            // by this time we know if v has a back edge to an earlier component
            low[*u] = std::cmp::min(low[*u], low[*v]);
            // this basically means that there is a cycle below u and lowest point
            // reachable in the dfs tree from v is below u
            if low[*v] > dfn[*u] {
                result.push(vec![*u, *v]);
            }
            continue;
        }
        // if the already visited neighbor is not the parent of v, then that means
        // there is a cycle
        if parent[*u] != Some(*v) {
            low[*u] = std::cmp::min(low[*u], dfn[*v]);
        }
    }
}

fn find_bridges(g: &GraphUnweightedAL) -> Vec<Vec<usize>> {
    let parent = &mut vec![None; g.len()];
    let dfn = &mut vec![g.len() + 1; g.len()];
    let low = &mut vec![g.len() + 1; g.len()];
    let vis = &mut vec![false; g.len()];
    let mut result = vec![];
    find_bridges_util(&0, &g, parent, vis, dfn, low, &mut 0, &mut result);
    result
}

impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let conn = connections.into_iter().map(|e| vec![e[0] as usize, e[1] as usize]).collect();
        find_bridges(&(make_graph(n as usize, &conn))).into_iter()
        .map(|e| vec![e[0] as i32, e[1] as i32]).collect()
    }
}