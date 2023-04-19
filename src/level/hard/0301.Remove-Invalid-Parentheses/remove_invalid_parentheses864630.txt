// https://leetcode.com/problems/remove-invalid-parentheses/solutions/864630/rust-translated-0ms-100/
impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        use std::iter::FromIterator;

        fn remove(s: &[char], ans: &mut Vec<String>, last_i: i32, last_j: i32, par: &[char]) {
            let mut stack = 0;
            for i in last_i..s.len() as i32 {
                if s[i as usize] == par[0] {
                    stack += 1;
                }
                if s[i as usize] == par[1] {
                    stack -= 1;
                }
                if stack >= 0 {
                    continue;
                }
                for j in last_j..=i {
                    if s[j as usize] == par[1] && (j == last_j || s[j as usize - 1] != par[1]) {
                        let mut s2 = Vec::<char>::new();
                        for k in 0..j as usize {
                            s2.push(s[k]);
                        }
                        for k in j as usize + 1..s.len() {
                            s2.push(s[k]);
                        }
                        remove(&s2, ans, i, j, par);
                    }
                }
                return;
            }
            let mut reversed = s.to_vec();
            reversed.reverse();
            if par[0] == '(' {
                remove(&reversed, ans, 0, 0, &[')', '(']);
            } else {
                ans.push(String::from_iter(reversed));
            }
        }
        let mut ans = vec![];
        remove(
            &s.chars().collect::<Vec<char>>(),
            &mut ans,
            0,
            0,
            &['(', ')'],
        );
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_invalid_parentheses() {
        assert_eq!(
            Solution::remove_invalid_parentheses("()())()".to_string()),
            vec!["()()()".to_owned(), "(())()".to_owned()]
        );
    }

    #[test]
    fn test_remove_invalid_parentheses_02() {
        assert_eq!(
            Solution::remove_invalid_parentheses("(a)())()".to_string()),
            vec!["(a)()()".to_owned(), "(a())()".to_owned()]
        );
    }

    #[test]
    fn test_remove_invalid_parentheses_03() {
        assert_eq!(
            Solution::remove_invalid_parentheses(")(".to_string()),
            vec!["".to_owned()]
        );
    }
}