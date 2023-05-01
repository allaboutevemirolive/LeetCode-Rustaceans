// https://leetcode.com/problems/generate-parentheses/solutions/3196004/rust-solution/
impl Solution {
    pub fn parenthesis_recurse_step(
        valids: &mut Vec<String>,
        bits: u16,
        size: u8,
        pairs: u8,
        max_pairs: u8,
    ) {
        if size - pairs > max_pairs {
            return;
        }

        if pairs == max_pairs {
            valids.push(
                (0..size)
                    .map(|shift| (40 + (bits >> shift & 1)) as u8 as char)
                    .collect::<String>(),
            );
            return;
        }

        if size > 2 * pairs as u8 {
            Solution::parenthesis_recurse_step(
                valids,
                bits | 1 << size,
                size + 1,
                pairs + 1,
                max_pairs,
            );
        }

        Solution::parenthesis_recurse_step(
            valids, 
            bits, 
            size + 1, 
            pairs, 
            max_pairs
        );
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut valids = Vec::new();
        Solution::parenthesis_recurse_step(&mut valids, 0, 0, 0, n as u8);
        valids
    }
}