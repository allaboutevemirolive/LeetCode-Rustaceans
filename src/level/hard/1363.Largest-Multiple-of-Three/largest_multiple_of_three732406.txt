// https://leetcode.com/problems/largest-multiple-of-three/solutions/732406/rust-translated/
impl Solution {
    pub fn largest_multiple_of_three(mut digits: Vec<i32>) -> String {
        let mut count = [0; 10];
        for d in digits {
            count[d as usize] += 1;
        }
        let mut r1 = count[1] + count[4] + count[7];
        let mut r2 = count[2] + count[5] + count[8];
        let r = (r1 + r2 * 2) % 3;
        if r == 1 {
            if r1 >= 1 {
                r1 -= 1
            } else {
                r2 -= 2
            }
        } else if r == 2 {
            if r2 >= 1 {
                r2 -= 1
            } else {
                r1 -= 2
            }
        }

        let mut ans = String::new();
        for d in (0..=9).rev() {
            match d {
                9 | 6 | 3 | 0 => {
                    while count[d] > 0 {
                        ans.push((b'0' + d as u8) as char);
                        count[d] -= 1;
                    }
                }
                1 | 4 | 7 => {
                    while count[d] > 0 && r1 > 0 {
                        ans.push((b'0' + d as u8) as char);
                        count[d] -= 1;
                        r1 -= 1
                    }
                }
                2 | 5 | 8 => {
                    while count[d] > 0 && r2 > 0 {
                        ans.push((b'0' + d as u8) as char);
                        count[d] -= 1;
                        r2 -= 1
                    }
                }
                _ => unreachable!(),
            }
        }
        if !ans.is_empty() && ans.as_bytes()[0] == b'0' {
            return "0".to_string();
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_multiple_of_three() {
        assert_eq!(
            Solution::largest_multiple_of_three(vec![8, 1, 9]),
            "981".to_string()
        )
    }

    #[test]
    fn test_largest_multiple_of_three_02() {
        assert_eq!(
            Solution::largest_multiple_of_three(vec![8, 6, 7, 1, 0]),
            "8760".to_string()
        )
    }

    #[test]
    fn test_largest_multiple_of_three_03() {
        assert_eq!(Solution::largest_multiple_of_three(vec![1]), "".to_string())
    }

    #[test]
    fn test_largest_multiple_of_three_04() {
        assert_eq!(
            Solution::largest_multiple_of_three(vec![0, 0, 0, 0, 0, 0]),
            "0".to_string()
        )
    }
}
