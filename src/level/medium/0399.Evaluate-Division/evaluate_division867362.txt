// https://leetcode.com/problems/evaluate-division/solutions/867362/rust-dfs-solution-0ms-explained/
use std::collections::{HashMap, HashSet};

type Graph = HashMap<String, HashMap<String, f64>>;

impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let graph = build_graph(equations, values);
        queries
            .into_iter()
            .map(|query| dfs(&graph, query, &mut HashSet::new(), None).map_or(-1.0, |v| v))
            .collect()
    }
}

fn build_graph(equations: Vec<Vec<String>>, values: Vec<f64>) -> Graph {
    equations
        .into_iter()
        .zip(values.into_iter())
        .fold(HashMap::new(), |mut acc, (eq, val)| {
            let entry = acc.entry(eq[0].clone()).or_default();
            entry.insert(eq[1].clone(), val);

            let entry = acc.entry(eq[1].clone()).or_default();
            entry.insert(eq[0].clone(), 1.0 / val);

            acc
        })
}

fn dfs(
    graph: &Graph,
    query: Vec<String>,
    visited: &mut HashSet<String>,
    current_sum: Option<f64>,
) -> Option<f64> {
    visited.insert(query[0].clone());

    // Check the the node exists in the graph
    let entry = graph.get(&query[0])?;

    // a / a = 1.0
    if query[0] == query[1] {
        return Some(1.0);
    }

    // If our current node has a direct route to our target, return it
    if let Some(answer) = entry.get(&query[1]) {
        return current_sum.map_or(Some(*answer), |s| Some(s * *answer));
    };

    // Otherwise perform a DFS of connected nodes (that we haven't already visited)
    for path in entry.keys() {
        if !visited.contains(path) {
            if let Some(val) = entry.get(path) {
                let result = dfs(
                    graph,
                    vec![path.clone(), query[1].clone()],
                    visited,
                    current_sum.map_or(Some(*val), |s| Some(s * *val)),
                );
                if result.is_some() {
                    return result;
                }
            }
        }
    }
    None
}