// https://leetcode.com/problems/evaluate-division/solutions/1549272/0ms-rust-solution-using-disjoint-sets/
impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let mut d = DS::new(equations.len() * 2);
        let mut i = 0;
        let mut answers = Vec::with_capacity(queries.len());
        for e in equations {
            d.union(&e[0], &e[1], values[i]);
            i+=1;
        }
        for q in queries {
            if !d.compare(&q[0], &q[1]) {
                answers.push(-1.0);
            }  else if &q[0] == &q[1] {
                answers.push(1.0);
            } else {
                let x = d.get_equation(&q[0]);
                let y = d.get_equation(&q[1]);

                answers.push(d.find_dividend(&x, &y));
            }
        }
        answers
    }
}

use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
struct Equation {
    pub id: String,
    pub equations: HashMap<String, f64>,
    pub edges: HashSet<String>,
    pub root: String
}

impl Equation {
    pub fn new(vertex: String) -> Equation {
        Equation {
            id: vertex.clone(),
            equations: HashMap::new(),
            edges: HashSet::new(),
            root: vertex.clone()
        }
    }
}

struct DS {
    roots: HashMap<String, Equation>,
    rank: HashMap<String, i32>,
    root_count: usize
}

impl DS {
    pub fn new(size: usize) -> DS {
        let roots = HashMap::with_capacity(size);
        let rank = HashMap::with_capacity(size);
        let root_count = size;

        DS {
            roots,
            rank,
            root_count
        }
    }

    pub fn find(& mut self, vertex: & str) -> String  {
        if !self.roots.contains_key(vertex) {
            self.roots.insert(vertex.to_string(), Equation::new(vertex.to_string()));
            vertex.to_string()
        } else {
            if self.roots.get(vertex).unwrap().root == vertex {
                vertex.to_string()
            } else {
                let new_root = self.roots.get(vertex).unwrap().root.clone();
                self.roots.get_mut(vertex).unwrap().root = self.find(&new_root).to_string();
                self.roots.get(vertex).unwrap().root.to_string()
            }
        }
    }

    pub fn union(&mut self, x: &str, y: &str, eq: f64) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            self.root_count -= 1;
            if self.rank.get(&root_x).unwrap_or(&-1) > self.rank.get(&root_y).unwrap_or(&-1) {
                self.roots.get_mut(&root_y).unwrap().root = root_x.to_string();
            } else if self.rank.get(&root_y).unwrap_or(&-1) > self.rank.get(&root_x).unwrap_or(&-1) {
                self.roots.get_mut(&root_x).unwrap().root = root_y.to_string();
            } else {
                if let Some(rank) = self.rank.get(&root_x) {
                    self.rank.insert(root_x.to_string(), rank+1);
                } else {
                    self.rank.insert(root_x.to_string(), 1);
                }
                self.roots.get_mut(&root_y).unwrap().root = root_x.to_string();
            }
        }
        self.roots.get_mut(x).unwrap().edges.insert(y.to_string());
        self.roots.get_mut(y).unwrap().edges.insert(x.to_string());
        self.roots.get_mut(x).unwrap().equations.insert(y.to_string(), eq);
        self.roots.get_mut(y).unwrap().equations.insert(x.to_string(), 1.0 / eq);
    }

    pub fn compare(&mut self, x: &str, y: &str) -> bool {
        if !self.exists(x) || !self.exists(y) {
            false
        } else {
            self.find(x) == self.find(y)
        }
    }

    pub fn get_equation(&self, x: &str) -> Equation {
        self.roots.get(x).unwrap().clone()
    }

    pub fn exists(&self, x: &str) -> bool {
        self.roots.contains_key(x)
    }

    pub fn find_dividend(&self, x: &Equation, y: &Equation) -> f64 {
        self.find_element(x, &y.id, HashSet::with_capacity(self.roots.len())) 
    }

    pub fn find_element(&self, x: &Equation, y: &str, mut seen: HashSet<String>) -> f64 {
        if x.edges.contains(y) {
            return *x.equations.get(y).unwrap();
        } else {
            for e in x.edges.clone() {
                if !seen.contains(e.as_str()) {
                    seen.insert(e.clone());
                    let ans = self.find_element(&self.get_equation(&e), y, seen.clone());
                    if ans != -1.0 {
                        return *x.equations.get(&e).unwrap() * ans;
                    }
                }
            }
        }
        -1.0
    }
}