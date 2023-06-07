// https://leetcode.com/problems/count-the-repetitions/solutions/791499/rust-translated-8ms-100/
impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        if n1 == 0 {
            return 0;
        };
        let mut indices = vec![0; n1 as usize + 1];
        let mut counts = vec![0; n1 as usize + 1];
        let mut index = 0;
        let mut count = 0;
        for i in 1..=n1 as usize {
            for j in 0..s1.len() {
                if s1.as_bytes()[j] == s2.as_bytes()[index] {
                    index += 1;
                }
                if index == s2.len() {
                    index = 0;
                    count += 1;
                }
            }
            counts[i] = count;
            indices[i] = index;
            for k in 0..i {
                if indices[k] == index {
                    let pre_count = counts[k];
                    let pattern_count = (n1 - k as i32) / (i - k) as i32 * (counts[i] - pre_count);
                    let remain_count = counts[k + (n1 as usize - k) % (i - k)] - pre_count;
                    return (pre_count + pattern_count + remain_count) / n2;
                }
            }
        }
        counts[n1 as usize] / n2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_max_repetitions() {
        assert_eq!(
            Solution::get_max_repetitions("acb".to_string(), 4, "ab".to_string(), 2),
            2
        )
    }

    #[test]
    fn test_get_max_repetitions_02() {
        assert_eq!(
            Solution::get_max_repetitions("aaa".to_string(), 20, "aaaaa".to_string(), 1),
            12
        )
    }

    #[test]
    fn test_get_max_repetitions_03() {
        assert_eq!(
            Solution::get_max_repetitions("bacaba".to_string(), 3, "abacab".to_string(), 1),
            2
        )
    }

    #[test]
    fn test_get_max_repetitions_04() {
        assert_eq!(
            Solution::get_max_repetitions("ecbafedcba".to_string(), 3, "abcdef".to_string(), 1),
            1
        )
    }
}