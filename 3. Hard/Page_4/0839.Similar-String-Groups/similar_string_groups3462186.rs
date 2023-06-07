// https://leetcode.com/problems/similar-string-groups/solutions/3462186/5ms-rust-python-o-w-2-l-connected-components-with-explanation/
use std::collections::HashSet;

impl Solution {

  fn connected_components(g: &Vec<Vec<usize>>) -> i32 {
    let mut last_used = 0;
    let mut is_used = vec![false; g.len()];
    let mut res = 0;

    while last_used < is_used.len() {
      let mut frontier = Vec::new();

      for i in last_used .. is_used.len() {
        if !is_used[i] {
          frontier = vec![i];
          is_used[i] = true;
          last_used = i + 1;
          break;
        }
      }
      if frontier.is_empty() {
        return res;
      }

      res += 1;      
      while !frontier.is_empty() {
        let mut new_frontier = Vec::new();
        for v in frontier {
          for w in &g[v] {
            if !is_used[*w] {
              new_frontier.push(*w);
              is_used[*w] = true;
            }
          }
        }
        frontier = new_frontier;
      }
    }

    return res;
  }

  fn is_similar(s1: &[u8], s2: &[u8]) -> bool {
    let mut num = 0;
    for i in 0 .. s1.len() {
      if s1[i] != s2[i] {
        num += 1;
        if num > 2 {
          return false;
        }
      }
    }
    return true;
  }

  fn unify(strs: &Vec<String>) -> Vec<&[u8]>{
    let mut res = Vec::new();
    for s in strs {
      res.push(s.as_bytes());
    }
    return res;


    let mut data = HashSet::new();
    for s in strs {
      data.insert(s);
    }

    let mut strs = Vec::with_capacity(data.len());
    for s in data {
      strs.push(s.as_bytes());
    }
    return strs;
  }

  fn build_graph(data: Vec<&[u8]>) -> Vec<Vec<usize>> {
    let n = data.len();
    let mut g = Vec::with_capacity(data.len());
    for i in 0 .. n {
      g.push(Vec::new());
    }

    for i in 0 .. n - 1 {
      for j in i + 1 .. n {
        if Self::is_similar(data[i], data[j]) {
          g[i].push(j);
          g[j].push(i);
        }
      }
    }

    return g;
  }

  pub fn num_similar_groups(strs: Vec<String>) -> i32 {
    let g = Self::build_graph(Self::unify(&strs));
    return Self::connected_components(&g);
  }
}