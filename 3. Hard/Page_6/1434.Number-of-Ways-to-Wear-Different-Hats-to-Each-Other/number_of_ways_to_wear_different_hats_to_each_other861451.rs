// https://leetcode.com/problems/number-of-ways-to-wear-different-hats-to-each-other/solutions/861451/rust-translated-4ms-100/
impl Solution {
    pub fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
        const MOD: i32 = 1_000_000_007;

        let n = hats.len(); // people count;
        let mut masks = vec![0; 1 << n];
        let mut people = vec![Vec::<i32>::new(); 40];
        for i in 0..n {
            for &h in &hats[i] {
                people[h as usize - 1].push(i as i32);
            }
        }

        masks[0] = 1;
        for i in 0..40 {
            for j in (0..1 << n).rev() {
                for &p in &people[i] {
                    if (j & (1 << p)) == 0 {
                        masks[j | (1 << p)] += masks[j];
                        masks[j | (1 << p)] %= MOD;
                    }
                }
            }
        }
        masks[(1 << n) - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_ways() {
        assert_eq!(
            Solution::number_ways(vec![vec![3, 4], vec![4, 5], vec![5]]),
            1
        );
    }

    #[test]
    fn test_number_ways_02() {
        assert_eq!(Solution::number_ways(vec![vec![3, 5, 1], vec![3, 5]]), 4);
    }

    #[test]
    fn test_number_ways_03() {
        assert_eq!(
            Solution::number_ways(vec![
                vec![1, 2, 3, 4],
                vec![1, 2, 3, 4],
                vec![1, 2, 3, 4],
                vec![1, 2, 3, 4]
            ]),
            24
        );
    }

    #[test]
    fn test_number_ways_04() {
        assert_eq!(
            Solution::number_ways(vec![
                vec![1, 2, 3],
                vec![2, 3, 5, 6],
                vec![1, 3, 7, 9],
                vec![1, 8, 9],
                vec![2, 5, 7]
            ]),
            111
        );
    }
}