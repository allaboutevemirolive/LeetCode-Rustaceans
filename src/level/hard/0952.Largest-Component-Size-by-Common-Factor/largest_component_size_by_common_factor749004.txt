// https://leetcode.com/problems/largest-component-size-by-common-factor/solutions/749004/rust-translated-union-find/
#[derive(Debug)]
struct UnionFind {
    id: Vec<i32>,
    size: Vec<i32>,
    n: i32,
    count: i32,
}

impl UnionFind {

    fn new(n: i32) -> UnionFind {
        assert!(n > 0);
        let count = n + 1;
        let id = (0..n + 1).collect();
        let size = std::iter::repeat(1).take(n as usize + 1).collect();
        UnionFind { id, size, n, count }
    }

    fn count(&self) -> i32 {
        self.count - 1
    }

    /** return the component id that the element x belongs to. */
    fn find(&mut self, p: i32) -> i32 {
        if self.id[p as usize] != p {
            self.id[p as usize] = self.find(self.id[p as usize]);
        }
        self.id[p as usize]
    }

    fn connected(&mut self, p: i32, q: i32) -> bool {
        self.find(p) == self.find(q)
    }

    /**
     * merge the two components that x, y belongs to respectively,
     * and return the merged component id as the result.
     */
    fn union(&mut self, p: i32, q: i32) -> i32 {
        let mut root_p = self.find(p);
        let mut root_q = self.find(q);
        if root_p == root_q {
            return root_q;
        }
        if self.size[root_p as usize] > self.size[root_q as usize] {
            std::mem::swap(&mut root_p, &mut root_q);
        }
        self.id[root_p as usize] = root_q;
        self.size[root_q as usize] += self.size[root_p as usize];
        self.size[root_p as usize] = 0;
        self.count -= 1;
        root_q
    }
}

impl Solution {
    pub fn largest_component_size(a: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        fn prime_decompose(mut n: i32) -> Vec<i32> {
            let mut ans = vec![];
            let mut x = 2;
            while n >= x * x {
                if n % x == 0 {
                    ans.push(x);
                    n /= x;
                } else {
                    x += 1;
                }
            }
            ans.push(n);
            ans
        }

        let mut max = a[0];
        for &x in &a {
            if x > max {
                max = x
            }
        }
        let mut uf = UnionFind::new(max);
        let mut map = HashMap::<i32, i32>::new();
        for &x in &a {
            let v = prime_decompose(x);
            map.insert(x, v[0]);
            for i in 0..v.len() - 1 {
                uf.union(v[i], v[i + 1]);
            }
        }
        let mut ans = 0;
        let mut count = HashMap::<i32, i32>::new();
        for &x in &a {
            let id = uf.find(*map.get(&x).unwrap());
            let c = count.entry(id).or_default();
            *c += 1;
            ans = std::cmp::max(ans, *c)
        }
        ans
    }
}