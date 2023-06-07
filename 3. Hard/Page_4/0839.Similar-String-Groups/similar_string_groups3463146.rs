// https://leetcode.com/problems/similar-string-groups/solutions/3463146/rust-union-find/
struct DSU {
    parents: Vec<usize>,
    ranks: Vec<usize>,
}

impl DSU {
    pub fn new(size: usize) -> Self {
        Self {
            parents: (0..size).collect(),
            ranks: vec![1; size],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if x != self.parents[x] {
            self.parents[x] = self.find(self.parents[x]);
        }
        self.parents[x]
    }

    pub fn union(&mut self, mut x: usize, mut y: usize) -> bool {
        x = self.find(x);
        y = self.find(y);

        if x == y {
            return false;
        }
        if self.ranks[x] < self.ranks[y] {
            std::mem::swap(&mut y, &mut x);
        }
        self.parents[y] = x;
        self.ranks[x] += self.ranks[y];
        
        true
    }
}

impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        // dsu for counting groups
        let n = strs.len();
        let mut uf = DSU::new(n);
        let items = strs.iter()
            .map(|s| s.as_bytes())
            .collect::<Vec<&[u8]>>();
        // iterate over all pairs O(N^2)

        let mut groups_count = n;
        for i in 0..n {
            for j in (i + 1)..n {
                // compare strings, union if they are similar
                if Self::are_similar(&items[i], &items[j]) {
                    if uf.union(i, j) {
                        groups_count -= 1;
                    }
                }
            }
        }

        groups_count as _
    }

    fn are_similar(left: &[u8], right: &[u8]) -> bool {
        let m = left.len();
        let mut diff = 0;
        for i in 0..m {
            if left[i] != right[i] {
                diff += 1;
            }
        }

        match diff {
            0 | 2 => true,
            _ => false,
        }
    }
}