// https://leetcode.com/problems/evaluate-division/solutions/2377460/rust-union-find-solution/
use std::collections::{HashMap};
impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let mut uf: HashMap<&str, (&str, f64)> = HashMap::new();
        let mut res = Vec::with_capacity(queries.len());
        
        for (equation, &val) in equations.iter().zip(values.iter()) {
            let left = &equation[0];
            let right = &equation[1];
            
            uf.entry(left).or_insert((left, 1.0));
            uf.entry(right).or_insert((right, 1.0));
            Self::union(&mut uf, left, right, val);
        }
        
        for query in queries.iter() {
            let left = &query[0];
            let right = &query[1];
            if !uf.contains_key(left.as_str()) || !uf.contains_key(right.as_str()) {
                res.push(-1.0);
            } else {
                let (r1, s1) = Self::find(&mut uf, left);
                let (r2, s2) = Self::find(&mut uf, right);
                if r1 != r2 {
                    res.push(-1.0);
                } else {
                    res.push(s1 / s2);
                }
            }
        }
        res
    }
    fn find<'a>(uf: &mut HashMap<&'a str, (&'a str, f64)>, node: &'a str) -> (&'a str, f64) {
        let (r1, s1) = *uf.get(node).unwrap(); 
        
        if uf.get(r1).unwrap().0 != r1 {
            let (r2, s2) = Self::find(uf, r1);
            *uf.get_mut(node).unwrap() = (r2, s2 * s1);
        }
        
        return *uf.get(node).unwrap();
    } 
    fn union<'a>(uf: &mut HashMap<&'a str, (&'a str, f64)>, left: &'a str, right: &'a str, val: f64) {
        let (r1, s1) = Self::find(uf, left);
        let (r2, s2) = Self::find(uf, right);
        
        if r1 != r2 {
            *uf.get_mut(r1).unwrap() = (r2, val * s2 / s1);
        }
    }
}