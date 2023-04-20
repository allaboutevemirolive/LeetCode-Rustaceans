// https://leetcode.com/problems/numbers-at-most-n-given-digit-set/solutions/829359/rust-translated-0ms-100/
impl Solution {
    pub fn at_most_n_given_digit_set(d: Vec<String>, n: i32) -> i32 {
        let b = d.len();
        let s = n.to_string();
        let k = s.len();
        let mut a = vec![0; k];
        let mut t = 0;
        for &ch in s.as_bytes() {
            let mut c_index = 0;
            let mut is_match = false;

            for i in 0..b {
                if ch >= d[i].as_bytes()[0] {
                    c_index = i + 1;
                }
                if ch == d[i].as_bytes()[0] {
                    is_match = true;
                }
            }
            a[t] = c_index;
            t += 1;
            if is_match {
                continue;
            }

            if c_index == 0 {
                // subtract 1
                for j in (1..t).rev() {
                    if a[j] > 0 {
                        break;
                    }
                    a[j] += b;
                    a[j - 1] -= 1;
                }
            }

            while t < k {
                a[t] = b;
                t += 1;
            }
            break;
        }
        let mut ans = 0;
        for x in a {
            ans = ans * b + x;
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at_most_n_given_digit_set() {
        assert_eq!(
            Solution::at_most_n_given_digit_set(
                vec![
                    "1".to_string(),
                    "3".to_string(),
                    "5".to_string(),
                    "7".to_string()
                ],
                100
            ),
            20
        )
    }

    #[test]
    fn test_at_most_n_given_digit_set_02() {
        assert_eq!(
            Solution::at_most_n_given_digit_set(
                vec!["1".to_string(), "4".to_string(), "9".to_string()],
                1_000_000_000
            ),
            29523
        )
    }

    #[test]
    fn test_at_most_n_given_digit_set_03() {
        assert_eq!(
            Solution::at_most_n_given_digit_set(vec!["7".to_string()], 8),
            1
        )
    }
}