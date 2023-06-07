// https://leetcode.com/problems/similar-string-groups/solutions/3462152/rust-union-find-solution/
impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        fn is_sim(s1: &Vec<char>, s2: &Vec<char>) -> bool {
            let mut diff = 0;
            for i in 0..s1.len() {
                if s1[i] != s2[i] {
                    diff += 1;
                }
                if diff > 2 {
                    return false;
                }
            }
            true
        }

        struct UF {
            cp: Vec<usize>,
            sz: Vec<usize>,
        }
        impl UF {
            fn new(n: usize) -> Self {
                Self {
                    cp: (0..n).collect(),
                    sz: vec![1; n],
                }
            }
            fn find(&mut self, x: usize) -> usize {
                if x != self.cp[x] {
                    self.cp[x] = self.find(self.cp[x]);
                }
                self.cp[x]
            }
            fn union(&mut self, x: usize, y: usize) {
                let mut x = self.find(x);
                let mut y = self.find(y);
                if x == y {
                    return;
                }
                if self.sz[x] < self.sz[y] {
                    std::mem::swap(&mut x, &mut y);
                }
                self.cp[y] = self.cp[x];
                self.sz[x] += self.sz[y];
            }
        }

        let mut uf = UF::new(strs.len());
        let strs = strs
            .into_iter()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut groups = strs.len() as i32;
        for i in 0..strs.len() {
            for j in i + 1..strs.len() {
                if is_sim(&strs[i], &strs[j]) && uf.find(i) != uf.find(j) {
                    uf.union(i, j);
                    groups -= 1;
                }
            }
        }

        groups
    }
}