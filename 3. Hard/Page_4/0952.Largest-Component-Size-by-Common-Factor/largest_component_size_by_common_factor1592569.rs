// https://leetcode.com/problems/largest-component-size-by-common-factor/solutions/1592569/rust-solution/
struct Dsu {
    p: Vec<i32>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self { p: vec![-1; n] }
    }

    fn get(&mut self, x: usize) -> usize {
        if self.p[x] < 0 {
            x
        } else {
            let ans = self.get(self.p[x] as _);
            self.p[x] = ans as _;
            ans
        }
    }

    fn unite(&mut self, x: usize, y: usize) -> bool {
        let x = self.get(x);
        let y = self.get(y);
        if x == y {
            return false;
        }

        let (x, y) = if self.p[x] < self.p[y] {
            (x, y)
        } else {
            (y, x)
        };

        self.p[y] += self.p[x];
        self.p[x] = y as _;

        true
    }

    fn max_comp_size(&self) -> usize {
        (-self.p.iter().filter(|&&x| x < 0).min().unwrap()) as _
    }
}

impl Solution {
    pub fn largest_component_size(nums: Vec<i32>) -> i32 {
        const MAX: usize = 100_001;

        let n = nums.len();

        let mut d = vec![MAX; MAX];
        for (i, num) in nums.into_iter().enumerate() {
            d[num as usize] = i;
        }

        let mut prime = vec![true; d.len()];
        let mut dsu = Dsu::new(n);
        for i in 2..prime.len() {
            if prime[i] {
                let mut prev = None;
                for (&dj, pj) in d[i..].iter().zip(&mut prime).step_by(i) {
                    *pj = false;
                    if dj == MAX {
                        continue;
                    }
                    if let Some(p) = prev {
                        dsu.unite(p, dj);
                    } else {
                        prev = Some(dj);
                    }
                }
            }
        }

        dsu.max_comp_size() as _
    }
}