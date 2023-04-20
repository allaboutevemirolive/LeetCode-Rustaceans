// https://leetcode.com/problems/sum-of-distances-in-tree/solutions/892069/rust-translated-8ms-100/
impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::HashSet;

        fn dfs(
            tree: &[HashSet<i32>],
            count: &mut Vec<i32>,
            ans: &mut Vec<i32>,
            root: i32,
            pre: i32,
        ) {
            for &i in &tree[root as usize] {
                if i == pre {
                    continue;
                }
                dfs(tree, count, ans, i, root);
                count[root as usize] += count[i as usize];
                ans[root as usize] += ans[i as usize] + count[i as usize];
            }
            count[root as usize] += 1;
        }

        fn dfs2(
            tree: &[HashSet<i32>],
            count: &mut Vec<i32>,
            ans: &mut Vec<i32>,
            root: i32,
            pre: i32,
        ) {
            for &i in &tree[root as usize] {
                if i == pre {
                    continue;
                }
                ans[i as usize] =
                    ans[root as usize] - count[i as usize] + count.len() as i32 - count[i as usize];
                dfs2(tree, count, ans, i, root);
            }
        }

        let mut tree = vec![HashSet::<i32>::new(); n as usize];
        let mut ans = vec![0; n as usize];
        let mut count = vec![0; n as usize];
        for e in &edges {
            tree[e[0] as usize].insert(e[1]);
            tree[e[1] as usize].insert(e[0]);
        }
        dfs(&tree, &mut count, &mut ans, 0, -1);
        dfs2(&tree, &mut count, &mut ans, 0, -1);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_distances_in_tree() {
        assert_eq!(
            Solution::sum_of_distances_in_tree(
                6,
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4], vec![2, 5]]
            ),
            vec![8, 12, 6, 10, 10, 10]
        );
    }
}