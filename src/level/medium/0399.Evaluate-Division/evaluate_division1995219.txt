// https://leetcode.com/problems/evaluate-division/solutions/1995219/rust-quick-union-with-path-compression/
use std::collections::HashMap;

#[derive(Debug)]
struct Helper {
    roots: HashMap<String,(String,f64)>,    
}

impl Helper {
    fn new() -> Self{
        Self{ 
            roots: HashMap::new(),
        }
    }
    
    fn find(&mut self, v: &str) -> (String, f64) {
        let mut maybe_updated = None;
        let maybe_rr = self.roots.get(v).map(|(s,r)| (s.to_string(),*r));
        if let Some((root,ratio)) = maybe_rr {
            if root == *v {
                return (root, 1.0)
            } else {
                let (root_y, ratio_y) = self.find(&root);
                // a / b = ratio
                // b / c = ratio2   
                // a / c = b * ratio * ratio2 / b = ratio* ratio2
                maybe_updated = Some((root_y, ratio*ratio_y));
            }
        } else  { // new vertex
            self.roots.insert(v.to_string(), (v.to_string(), 1.0));
            return (v.to_string(), 1.0)
        }
        if let Some(updated) = maybe_updated {
            self.roots.insert(v.to_string(),updated.clone());
            return updated
        } else {
            panic!("dead branch");
        }
    }
    
    fn insert(&mut self, x: &str, y: &str, ratio: f64) {
        // x / rx = a
        // y / ry = b
        // x / y = v
        // y / rx = y / x * a = a / v 
        // x / ry = x / y * b = v * b
        // x = a, y = b, v = 2 => a->(a,1), b->(b,1) ; a->(b, 2), b->(b,1)
        // x = b, y = c, v = 3 => b->(b,1) ; c->(c,1); b->(c,3), c->(c,1)
        // find(a) => a->(b,2); b->(c,3); c->(c,1); a->(c,6)
        let (root_x, ratio_x) = self.find(x);
        let (root_y, ratio_y) = self.find(y);
        if root_x != root_y { // if they have the same root, job done
            self.roots.insert(x.to_string(),(root_y.clone(),ratio*ratio_y));
            self.roots.insert(root_x,(root_y,ratio*ratio_y/ratio_x));
        }
    }
    
    fn eval(&mut self, x:&str, y:&str) -> f64 {
        const Fail:f64 = -1.0;
        if !self.roots.contains_key(x) || !self.roots.contains_key(y) { // one of the two variable is unknown 
            return Fail
        } 
        let (root_x, ratio_x) = self.find(x);
        let (root_y, ratio_y) = self.find(y);
        if root_x == root_y {
            // x / rx = a
            // y / rx = b
            // x / y = a * rx / ( b * rx) = a / b
            ratio_x / ratio_y
        } else {
            Fail
        }
    }
}


impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        // equations is a list of edges
        // traverse the list of edge to connect all nodes
        // compute the equations
        let mut helper = Helper::new();
        for (e,v) in equations.iter().zip(values.iter()) {
            helper.insert(&e[0], &e[1], *v);
        }
        queries.iter().map(|v| helper.eval(&v[0], &v[1])).collect()
    }
}