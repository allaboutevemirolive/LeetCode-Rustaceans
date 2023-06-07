// https://leetcode.com/problems/last-substring-in-lexicographical-order/solutions/839717/rust-translated-4ms-100/
impl Solution {
    pub fn last_substring(s: String) -> String {
        let mut i = 0;
        let mut j = 1;
        let mut k = 0;
        let n = s.len();
        let v = s.as_bytes();
        while j + k < n {
            if v[i + k] == v[j + k] {
                k += 1;
                continue;
            } else if v[i + k] > v[j + k] {
                j = j + k + 1
            } else {
                i = std::cmp::max(i + k + 1, j);
                j = i + 1;
            }
            k = 0;
        }
        s[i as usize..].to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_last_substring() {
        assert_eq!(
            Solution::last_substring("abab".to_string()),
            "bab".to_string()
        );
    }

    #[test]
    fn test_last_substring_02() {
        assert_eq!(
            Solution::last_substring("leetcode".to_string()),
            "tcode".to_string()
        );
    }
}