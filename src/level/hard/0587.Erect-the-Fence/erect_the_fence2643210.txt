// https://leetcode.com/problems/erect-the-fence/solutions/2643210/rust/
use std::collections::HashSet;

impl Solution {
    pub fn minus(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> { vec![a[0] - b[0], a[1] - b[1]] }
    pub fn dot(a: &Vec<i32>, b: &Vec<i32>) -> i32 { a[0] * b[0] + a[1] * b[1] }
    pub fn distance(a: &Vec<i32>) -> f64 { (Solution::dot(a, a) as f64).sqrt() }
    pub fn angle(a: &Vec<i32>, pivot: &Vec<i32>, b: &Vec<i32>) -> f64 {
        let bo = Solution::minus(&b, &pivot);
        let ao = Solution::minus(&a, &pivot);
        Solution::dot(&bo, &ao) as f64 / Solution::distance(&ao) / Solution::distance(&bo)
    }
    pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut trees = trees.clone();
        if trees.len() <= 3 { return trees }
        let mut ret = Vec::new();
        let (mut left_idx, mut left_value, mut y_value ) = (0, 101, 101);
        for (i, tree) in trees.iter().enumerate() {
            if tree[0] < left_value || (tree[0] == left_value && tree[1] < y_value) {
                left_idx = i; left_value = tree[0]; y_value = tree[1];
            }
        }

        let mut pivot = &trees[left_idx].clone();
        let mut pivot_idx = left_idx;
        ret.push(pivot.clone());

        let mut reference = &vec![pivot[0], pivot[1]+101];
        let mut ref_idx: usize = 99999;

        let mut seen = HashSet::new();
        // seen.insert(left_idx);

        loop {
            let (mut next_idx, mut m_angle, mut m_dis) = (0 as usize, 10.0f64, 1000f64);
            for (i, tree) in trees.iter().enumerate() {
                if i == pivot_idx || i == ref_idx || seen.contains(&i) { continue; }
                let cur_angle = Solution::angle(reference, pivot, tree);
                let cur_dis = Solution::distance(&Solution::minus(tree, pivot));
                // println!("{tree:?} -- {cur_angle} {cur_dis}");
                if cur_angle+0.00001 < m_angle || (cur_angle < m_angle+0.00001 && cur_dis < m_dis) {
                    // println!("Select {i}");
                    m_angle = cur_angle;
                    m_dis = cur_dis;
                    next_idx = i;
                }
            }
            if next_idx == left_idx { break; }
            seen.insert(next_idx);
            reference = pivot;
            ref_idx = pivot_idx;
            pivot = &trees[next_idx];
            pivot_idx = next_idx;
            // println!("Do {:?}", trees[next_idx]);
            ret.push(trees[next_idx].clone());
        }
        ret
    }
}