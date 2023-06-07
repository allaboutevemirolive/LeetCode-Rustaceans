// https://leetcode.com/problems/maximum-score-words-formed-by-letters/solutions/860586/rust-translated-backtrack-0ms-100/
impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        fn back_track(words: &[String], count: &mut [i32], score: &[i32], index: usize) -> i32 {
            let mut max = 0;
            for i in index..words.len() {
                let mut res = 0;
                let mut is_valid = true;
                for &ch in words[i].as_bytes() {
                    count[(ch as u8 - b'a') as usize] -= 1;
                    res += score[(ch as u8 - b'a') as usize];
                    if count[(ch as u8 - b'a') as usize] < 0 {
                        is_valid = false;
                    }
                }
                if is_valid {
                    res += back_track(words, count, score, i + 1);
                    if res > max {
                        max = res;
                    }
                }
                for &ch in words[i].as_bytes() {
                    count[(ch as u8 - b'a') as usize] += 1;
                }
            }
            max
        }
        let mut count = vec![0; 26];
        for &ch in &letters {
            count[(ch as u8 - b'a') as usize] += 1;
        }
        back_track(&words, &mut count, &score, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_score_words() {
        assert_eq!(
            Solution::max_score_words(
                vec![
                    "dog".to_owned(),
                    "cat".to_owned(),
                    "dad".to_owned(),
                    "good".to_owned()
                ],
                vec!['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'],
                vec![1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ),
            23
        );
    }

    #[test]
    fn test_max_score_words_02() {
        assert_eq!(
            Solution::max_score_words(
                vec![
                    "xxxz".to_owned(),
                    "ax".to_owned(),
                    "bx".to_owned(),
                    "cx".to_owned()
                ],
                vec!['z', 'a', 'b', 'c', 'x', 'x', 'x'],
                vec![4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10]
            ),
            27
        );
    }

    #[test]
    fn test_max_score_words_03() {
        assert_eq!(
            Solution::max_score_words(
                vec!["leetcode".to_owned()],
                vec!['l', 'e', 't', 'c', 'o', 'd'],
                vec![0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0]
            ),
            0
        );
    }
}