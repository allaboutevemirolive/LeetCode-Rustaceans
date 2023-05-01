// https://leetcode.com/problems/evaluate-division/solutions/3419337/rust-solution/
impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        fn search(start: String, end: String, graph: &HashMap<String, HashMap<String, f64>>) -> f64 {
            if !graph.contains_key(&start) {
                return -1.;
            }
            use std::collections::HashSet;
            let mut stack = vec![];
            let mut seen = HashSet::new();
            stack.push((&start, 1.));
            seen.insert(&start);
            while let Some((v, ratio)) = stack.pop() {
                if v == &end {
                    return ratio;
                }
                for (u, r) in &graph[v] {
                    if !seen.contains(u) {
                        seen.insert(u);
                        stack.push((u, ratio * r))
                    }
                }
            }
            -1.
        }
        use std::collections::HashMap;
        let mut graph = HashMap::new();
        for (i, equ) in equations.into_iter().enumerate() {
            let num = equ[0].clone();
            let div = equ[1].clone();
            graph
                .entry(num.clone())
                .or_insert(HashMap::new())
                .insert(div.clone(), values[i]);
            graph
                .entry(div.clone())
                .or_insert(HashMap::new())
                .insert(num.clone(), 1. / values[i]);
        }
        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            ans.push(search(q[0].clone(), q[1].clone(), &graph))
        }
        ans        
    }
}