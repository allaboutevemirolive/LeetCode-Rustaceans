// https://leetcode.com/problems/falling-squares/solutions/2811646/rust-segment-tree-with-print-function-for-trouble-shooting/
use std::collections::BTreeSet;
use std::collections::HashMap;
impl Solution {
    pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        let mut s = BTreeSet::new();
        for position in &positions {
            s.insert(position[0]);
            s.insert(position[0] + position[1] - 1);
        }

        let s = s.iter().map(|a| *a).collect::<Vec<i32>>(); 
        let n = s.len();
        let mut mp = HashMap::new();
        for i in 0 .. s.len() { mp.insert(s[i], i); }
        // Lazy Propagation Segment Tree                     
        let mut tree = vec![(0, 0); 4 * n];
        let mut ret = vec![];
        let mut peak = 0;

        for p in positions {
            let (u, v) = (*mp.get(&p[0]).unwrap(), *mp.get(&(p[0] + p[1] - 1)).unwrap());
            let height = p[1] + Self::query(1, 0, n - 1, u, v, &mut tree);
            peak = peak.max(height);
            ret.push(peak);
            Self::update(1, 0, n - 1, u, v, height, &mut tree);
            //Self::print(1, 0, n - 1, &tree);
            //println![""];        
        }

        ret
    }

    fn print(u: usize, left: usize, right: usize, tree: &Vec<(i32, i32)>) {
        println!["u: {}, left: {}, right: {}, tree[u].0: {}, tree[u].1 {} ",
                 u, left, right, tree[u].0, tree[u].1];
        if left == right { return }

        let mid = left + (right - left) / 2;
        Self::print(2 * u, left, mid, tree);
        Self::print(2 * u + 1, mid + 1, right, tree);
    }

    fn query(u: usize, left: usize, right: usize, l: usize, r: usize, tree: &mut Vec<(i32, i32)>) -> i32 {
        if l <= left && right <= r {
            return tree[u].0.max(tree[u].1) 
        }
        if left > r || right < l || left == right { return 0 }

        let mid = left + (right - left) / 2;

        if tree[u].0 > 0 {
            tree[u].1 = tree[u].1.max(tree[u].0);
            
            tree[2 * u].0 = tree[2 * u].0.max(tree[u].0);
            tree[2 * u + 1].0 = tree[2 * u + 1].0.max(tree[u].0);

            tree[u].0 = 0;
        }

        let ret1 = Self::query(2 * u, left, mid, l, r, tree);
        let ret2 = Self::query(2 * u + 1, mid + 1, right, l, r, tree);

        tree[u].1 = tree[u].1.max(tree[2 * u].1);
        tree[u].1 = tree[u].1.max(tree[2 * u + 1].1);
          
        ret1.max(ret2)
    }

    fn update(u: usize, left: usize, right: usize, l: usize, r: usize, h: i32, tree: &mut Vec<(i32, i32)>) {
        if l <= left && right <= r {
            tree[u].0 = tree[u].0.max(h);
            return
        }
        if left > r || right < l || left == right { return }
        
        let mid = left + (right - left) / 2;

        Self::update(2 * u, left, mid, l, r, h, tree);
        Self::update(2 * u + 1, mid + 1, right, l, r, h, tree);
        tree[u].1 = tree[u].1.max(tree[2 * u].1);
        tree[u].1 = tree[u].1.max(tree[2 * u + 1].1);

        tree[u].1 = tree[u].1.max(tree[2 * u].0);
        tree[u].1 = tree[u].1.max(tree[2 * u + 1].0);
    }
}