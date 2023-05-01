// https://leetcode.com/problems/triangle/solutions/2001682/rust-concise-functional-style/
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        triangle
            .into_iter()
            .rev()
            .reduce(|prev, mut curr| {
                curr.iter_mut()
                    .zip(prev.windows(2).map(|w| w[0].min(w[1])))
                    .for_each(|(p, c)| *p += c);
                curr
            })
            .unwrap()[0]
    }
}