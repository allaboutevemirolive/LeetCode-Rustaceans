// https://leetcode.com/problems/longest-chunked-palindrome-decomposition/solutions/839779/rust-translated-0ms-100/
impl Solution {
    pub fn longest_decomposition(text: String) -> i32 {
        for i in 1..=text.len() / 2 {
            if text[0..i] == text[text.len() - i..] {
                return 2 + Self::longest_decomposition(text[i..text.len() - i].to_owned());
            }
        }
        if text.len() == 0 {
            0
        } else {
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_decomposition() {
        assert_eq!(
            Solution::longest_decomposition("ghiabcdefhelloadamhelloabcdefghi".to_string()),
            7
        );
    }

    #[test]
    fn test_longest_decomposition_02() {
        assert_eq!(Solution::longest_decomposition("merchant".to_string()), 1);
    }

    #[test]
    fn test_longest_decomposition_03() {
        assert_eq!(
            Solution::longest_decomposition("antaprezatepzapreanta".to_string()),
            11
        );
    }

    #[test]
    fn test_longest_decomposition_04() {
        assert_eq!(Solution::longest_decomposition("aaa".to_string()), 3);
    }
}