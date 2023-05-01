// https://leetcode.com/problems/generate-parentheses/solutions/2134218/rust-iterative-one-expression/
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        (1..=n)
            .chain((0..n).rev())
            .fold(vec![(String::new(), 0)], |acc, max_height| {
                acc.into_iter()
                    .flat_map(|(stem, height)| {
                        [(1, "("), (-1, ")")][(if height < max_height { 0 } else { 1 })
                            ..(if height > 0 { 2 } else { 1 })]
                            .iter()
                            .map(move |(offset, paren)| (stem.clone() + paren, height + *offset))
                    })
                    .collect()
            })
            .into_iter()
            .map(|(sol, _)| sol)
            .collect()
    }
}