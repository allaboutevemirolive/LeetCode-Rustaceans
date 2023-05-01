// https://leetcode.com/problems/evaluate-division/solutions/1993164/rust-with-union-find/
use std::collections::HashMap;

impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {    
        let mut union_find = UnionFindWeight::new();
        for i in 0..values.len() {
            union_find.union_weight(&equations[i][0], &equations[i][1], values[i]);
        }
        queries.into_iter()
        .map(|v| {
            let (a_root, a_ratio) = union_find.find_root_and_ratio(&v[0]);
            let (b_root, b_ratio) = union_find.find_root_and_ratio(&v[1]);
            if a_root != b_root || a_ratio.is_none() || b_ratio.is_none() {
                -1.0
            }
            else {
                b_ratio.unwrap() / a_ratio.unwrap()
            }
        }).collect()
    }
}

struct UnionFindWeight {
    parent: HashMap<String, String>,
    ratio: HashMap<String, f64>,
}

impl UnionFindWeight {
    fn new() -> Self {
        Self {
            parent: HashMap::new(),
            ratio: HashMap::new(),
        }
    }

    fn find_root_and_ratio(&self, s: &String) -> (String, Option<f64>) {
        let mut s = s;
        let mut weight = 1.0;
        loop {
            match self.parent.get(s) {
                Some(s2) => if *s == *s2 {
                    return (s.to_owned(), Some(weight)); // weight = value of root(s) / value of s
                }
                else {
                    weight *= self.ratio.get(s).unwrap();
                    s = s2;
                }
                None => return (s.to_owned(), None),
            }
        }
    }

    fn union_weight(&mut self, a: &String, b: &String, value: f64) {
        let (a_root, a_ratio) = self.find_root_and_ratio(a);
        let (b_root, b_ratio) = self.find_root_and_ratio(b);
        if a_ratio.is_none() {
            self.parent.insert(a_root.clone(), a_root.clone());
        } // ratio[a_root] isn't needed
        self.parent.insert(b_root.clone(), a_root.clone());
        self.ratio.insert(b_root.clone(), value * a_ratio.unwrap_or(1.0) / b_ratio.unwrap_or(1.0));
    }
}