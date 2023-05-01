// https://leetcode.com/problems/evaluate-division/solutions/1994130/rust-iterative-dfs-0-ms/
use std::collections::HashMap;

impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let mut graph = HashMap::<&String, Vec<(&String, f64)>>::new();
        equations.iter().zip(values).for_each(|(identifiers, dividend)| {
            graph.entry(&identifiers[0]).or_default().push((&identifiers[1], dividend));
            graph.entry(&identifiers[1]).or_default().push((&identifiers[0], 1.0/dividend));
        });

        let mut result = Vec::with_capacity(queries.len());
        let mut visited = graph.keys().map(|k| (*k, false)).collect::<HashMap<_,_>>();
        let mut stack = vec![];

        for q in queries.iter() {
            let start = &q[0];
            let end = &q[1];
            result.push(
            if !graph.contains_key(start) || !graph.contains_key(end) {
                -1.0
            } else if start == end {
                1.0
            } else {
                stack.push((start, 1.0));

                let mut r = -1.0;

                while let Some((id1, product))= stack.pop() {
                    let v = visited.get_mut(id1).unwrap();
                    if *v {
                        continue;
                    }

                    if id1 == end {
                        r = product;
                        break;
                    }

                    *v = true;

                    let edges = graph.get(id1).unwrap();
                    for (id2, factor) in edges.iter() {
                        stack.push((id2, product * factor));
                    }
                }

                r
            });
            stack.clear();
            visited.values_mut().for_each(|v| *v = false);

        }

        result
    }
}