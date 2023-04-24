// https://leetcode.com/problems/online-majority-element-in-subarray/solutions/761571/rust-translated/
use std::collections::HashMap;

struct MajorityChecker {
    indices: HashMap<i32, Vec<i32>>,
    tree: Vec<Vec<i32>>,
    size: i32,
}

impl MajorityChecker {
    fn new(arr: Vec<i32>) -> Self {
        let size = arr.len();
        let mut tree = vec![vec![0, 0]; size * 4];
        Self::build_tree(&arr, &mut tree, 0, 0, (size - 1) as i32);
        let mut indices = HashMap::<i32, Vec<i32>>::new();
        for i in 0..size {
            indices.entry(arr[i]).or_default().push(i as i32);
        }

        MajorityChecker {
            tree,
            indices,
            size: size as i32,
        }
    }

    fn query(&mut self, left: i32, right: i32, threshold: i32) -> i32 {
        if self.size == 0 {
            return -1;
        }
        let node = Self::find_max(&mut self.tree, 0, 0, self.size - 1, left, right);
        if node[1] == 0 {
            return -1;
        }
        let idx_arr = self.indices.get(&node[0]).unwrap();
        let it1 = lower_bound(&idx_arr,&left);
        let it2 = upper_bound(&idx_arr, &right);
        let cnt = it2 - it1;
        if cnt >= threshold {
            node[0]
        } else {
            -1
        }
    }

    fn find_max(
        tree: &mut Vec<Vec<i32>>,
        root: i32,
        left: i32,
        right: i32,
        left_limited: i32,
        right_limited: i32,
    ) -> Vec<i32> {
        let node = tree[root as usize].clone();
        if left >= left_limited && right <= right_limited {
            return node;
        }
        let mid = (left + right) / 2;
        let rl = root * 2 + 1;
        let rr = rl + 1;
        if mid < left_limited {
            return Self::find_max(tree, rr, mid + 1, right, left_limited, right_limited);
        }
        if mid + 1 > right_limited {
            return Self::find_max(tree, rl, left, mid, left_limited, right_limited);
        }
        let mut t = vec![0, 0];
        return Self::merge(
            &mut t,
            Self::find_max(tree, rl, left, mid, left_limited, right_limited),
            Self::find_max(tree, rr, mid + 1, right, left_limited, right_limited),
        );
    }

    fn build_tree(arr: &[i32], tree: &mut Vec<Vec<i32>>, root: i32, left: i32, right: i32) {
        if left == right {
            tree[root as usize][0] = arr[left as usize];
            tree[root as usize][1] = 1;
            return;
        }
        let mid = (left + right) / 2;
        let rl = root * 2 + 1;
        let rr = rl + 1;
        Self::build_tree(arr, tree, rl, left, mid);
        Self::build_tree(arr, tree, rr, mid + 1, right);
        let l_tree = tree[rl as usize].clone();
        let r_tree = tree[rr as usize].clone();
        Self::merge(&mut tree[root as usize], l_tree, r_tree);
    }

    fn merge(root: &mut Vec<i32>, left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
        if left[0] == right[0] {
            root[0] = left[0];
            root[1] = left[1] + right[1];
        } else if left[1] > right[1] {
            root[0] = left[0];
            root[1] = left[1] - right[1];
        } else {
            root[0] = right[0];
            root[1] = right[1] - left[1];
        }
        root.clone()
    }
}

fn lower_bound(v: &[i32], e: &i32) -> i32 {
    let mut left = 0;
    let mut right = v.len();
    while left < right {
        let mid = left + (right - left) / 2;
        if v[mid] >= *e {
            right = mid;
        } else {
            left += 1
        }
    }
    left as i32
}

fn upper_bound(v: &[i32], e: &i32) -> i32 {
    let mut left = 0;
    let mut right = v.len();
    while left < right {
        let mid = left + (right - left) / 2;
        if v[mid] > *e {
            right = mid;
        } else {
            left += 1
        }
    }
    right as i32

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query() {
        let a = [10, 11, 13, 13, 15];

        assert_eq!(super::lower_bound(&a, &9), 0);
        assert_eq!(super::lower_bound(&a, &10), 0);
        assert_eq!(super::lower_bound(&a, &11), 1);
        assert_eq!(super::lower_bound(&a, &12), 2);
        assert_eq!(super::lower_bound(&a, &13), 2);
        assert_eq!(super::lower_bound(&a, &14), 4);
        assert_eq!(super::lower_bound(&a, &15), 4);
        assert_eq!(super::lower_bound(&a, &16), 5);


        assert_eq!(super::upper_bound(&a, &9), 0);
        assert_eq!(super::upper_bound(&a, &10), 1);
        assert_eq!(super::upper_bound(&a, &11), 2);
        assert_eq!(super::upper_bound(&a, &12), 2);
        assert_eq!(super::upper_bound(&a, &13), 4);
        assert_eq!(super::upper_bound(&a, &14), 4);
        assert_eq!(super::upper_bound(&a, &15), 5);
        assert_eq!(super::upper_bound(&a, &16), 5);

        let mut majority_checker = MajorityChecker::new(vec![1, 1, 2, 2, 1, 1]);
        assert_eq!(majority_checker.query(0, 5, 4), 1); // returns 1
        assert_eq!(majority_checker.query(0, 3, 3), -1); // returns -1
        assert_eq!(majority_checker.query(2, 3, 2), 2); // returns 2
    }
}