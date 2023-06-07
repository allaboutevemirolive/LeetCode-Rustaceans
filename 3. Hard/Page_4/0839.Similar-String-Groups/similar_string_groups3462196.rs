// https://leetcode.com/problems/similar-string-groups/solutions/3462196/rust-union-find-solution/
impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let n = strs.len();
        let mut total = n;
        let mut uf = UnionFind::new(n);

        for i in 0..n {
            for j in i + 1..n {
                if Solution::is_similar(&strs[i], &strs[j]) {
                    if uf.union(i, j) {
                        total -= 1;
                    }
                }
            }
        }

        total as i32
    }

    fn is_similar(s1: &String, s2: &String) -> bool {
        s1.chars().zip(s2.chars()).map(|(a, b)| {
            (a != b) as i32
        }).sum::<i32>() <= 2
    }
}

struct UnionFind {
    root: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut root = vec![0; n];
        let mut size = vec![0; n];

        for i in 0..n {
            root[i] = i;
            size[i] = 1;
        }

        UnionFind { root, size }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.root[x] != x {
            self.root[x] = Self::find(self, self.root[x]);
        }

        self.root[x]
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = Self::find(self, x);
        let root_y = Self::find(self, y);

        if root_x == root_y {
            return false;
        }

        if self.size[root_x] < self.size[root_y] {
            self.root[root_x] = self.root[root_y];
            self.size[root_y] += self.size[root_x];
        } else {
            self.root[root_y] = self.root[root_x];
            self.size[root_x] += self.root[root_y];
        }

        true
    }
}