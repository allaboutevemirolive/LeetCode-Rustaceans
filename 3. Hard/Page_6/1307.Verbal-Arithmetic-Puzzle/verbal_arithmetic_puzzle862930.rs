// https://leetcode.com/problems/verbal-arithmetic-puzzle/solutions/862930/rust-translated-72ms-100/
/*
https://leetcode.com/problems/verbal-arithmetic-puzzle/discuss/463916/C%2B%2B-12ms-DFS-and-Backtracking-and-Prunning-Strategy
*/
impl Solution {
    pub fn is_solvable(words: Vec<String>, result: String) -> bool {
        use std::collections::HashMap;

        fn helper(
            words: &[Vec<char>],
            result: &[char],
            col: usize,
            row: usize,
            sum: i32,
            c2i: &mut HashMap<char, i32>,
            i2c: &mut HashMap<i32, char>,
        ) -> bool {
            if col == result.len() {
                return sum == 0
                    && if col > 1 {
                        c2i[&result[col - 1]] != 0
                    } else {
                        true
                    };
            }
            if row == words.len() {
                return if !c2i.contains_key(&result[col]) && !i2c.contains_key(&(sum % 10)) {
                    if sum % 10 == 0 && col + 1 == result.len() {
                        return false;
                    }
                    *c2i.entry(result[col]).or_default() = sum % 10;
                    *i2c.entry(sum % 10).or_default() = result[col as usize];
                    let tmp = helper(words, result, col + 1, 0, sum / 10, c2i, i2c);
                    c2i.remove(&result[col]);
                    i2c.remove(&(sum % 10));
                    tmp
                } else if *c2i.get(&result[col]).unwrap_or(&-1) == sum % 10 {
                    helper(words, result, col + 1, 0, sum / 10, c2i, i2c)
                } else {
                    false
                };
            }

            if col >= words[row].len() {
                return helper(words, result, col, row + 1, sum, c2i, i2c);
            }
            if c2i.contains_key(&words[row][col]) {
                if col + 1 == words[row].len() && words[row].len() > 1 && c2i[&words[row][col]] == 0
                {
                    return false;
                }
                return helper(
                    words,
                    result,
                    col,
                    row + 1,
                    sum + c2i[&words[row][col]],
                    c2i,
                    i2c,
                );
            }
            for i in 0..10 {
                if col + 1 == words[row].len() && i == 0 && words[row].len() > 1 {
                    continue;
                }
                if i2c.contains_key(&(i as i32)) {
                    continue;
                }
                *c2i.entry(words[row][col]).or_default() = i as i32;
                *i2c.entry(i as i32).or_default() = words[row as usize][col];
                let tmp = helper(words, result, col, row + 1, sum + i, c2i, i2c);
                c2i.remove(&words[row][col]);
                i2c.remove(&(i as i32));
                if tmp {
                    return true;
                }
            }
            false
        }

        for word in &words {
            if word.len() > result.len() {
                return false;
            }
        }
        let mut equations = vec![];
        for word in &words {
            equations.push(word.chars().rev().collect::<Vec<char>>());
        }
        let ans = result.chars().rev().collect::<Vec<char>>();
        let mut c2i = HashMap::<char, i32>::new(); // char -> i32
        let mut i2c = HashMap::<i32, char>::new(); // i32 -> char
        helper(&equations, &ans, 0, 0, 0, &mut c2i, &mut i2c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_solvable() {
        assert_eq!(
            Solution::is_solvable(
                vec!["SEND".to_owned(), "MORE".to_owned()],
                "MONEY".to_owned()
            ),
            true
        );
    }

    #[test]
    fn test_is_solvable_02() {
        assert_eq!(
            Solution::is_solvable(
                vec!["SIX".to_owned(), "SEVEN".to_owned(), "SEVEN".to_owned()],
                "TWENTY".to_owned()
            ),
            true
        );
    }

    #[test]
    fn test_is_solvable_03() {
        assert_eq!(
            Solution::is_solvable(
                vec!["THIS".to_owned(), "IS".to_owned(), "TOO".to_owned()],
                "FUNNY".to_owned()
            ),
            true
        );
    }

    #[test]
    fn test_is_solvable_04() {
        assert_eq!(
            Solution::is_solvable(
                vec!["LEET".to_owned(), "CODE".to_owned()],
                "POINT".to_owned()
            ),
            false
        );
    }

    #[test]
    fn test_is_solvable_05() {
        assert_eq!(
            Solution::is_solvable(
                vec![
                    "I".to_owned(),
                    "THINK".to_owned(),
                    "IT".to_owned(),
                    "BE".to_owned(),
                    "THINE".to_owned()
                ],
                "INDEED".to_owned()
            ),
            true
        );
    }

    #[test]
    fn test_is_solvable_06() {
        assert_eq!(
            Solution::is_solvable(vec!["A".to_owned(), "A".to_owned(),], "AA".to_owned()),
            false
        );
    }

    #[test]
    fn test_is_solvable_07() {
        assert_eq!(
            Solution::is_solvable(vec!["A".to_owned(), "A".to_owned(),], "A".to_owned()),
            true
        );
    }

    #[test]
    fn test_is_solvable_08() {
        assert_eq!(
            Solution::is_solvable(vec!["A".to_owned(), "B".to_owned(),], "A".to_owned()),
            true
        );
    }
}
