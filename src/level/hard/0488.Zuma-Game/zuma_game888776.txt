// https://leetcode.com/problems/zuma-game/solutions/888776/rust-translated-0ms-100/
impl Solution {
    pub fn find_min_step(mut board: String, hand: String) -> i32 {
        const MAX_COUNT: i32 = 6;
        fn remove_consecutive(s: &str) -> String {
            let mut i = 0;
            for j in 0..s.len() {
                if s.as_bytes()[j] == s.as_bytes()[i] {
                    continue;
                }
                if j - i >= 3 {
                    let mut s2 = String::from(&s[..i]);
                    s2.push_str(&s[j..]);
                    return remove_consecutive(&s2);
                } else {
                    i = j;
                }
            }
            s.to_owned()
        }

        fn dfs(str: &str, h: &mut Vec<i32>, remove: bool) -> i32 {
            let s = remove_consecutive(str);
            let s = if remove { &s } else { str };
            if s == "#" {
                return 0;
            }
            let mut ans = MAX_COUNT;
            let mut i = 0;
            for j in 0..s.len() {
                if s.as_bytes()[j] == s.as_bytes()[i] {
                    continue;
                }
                let needed = 3 - (j - i) as i32;
                if needed >= 0 && h[(s.as_bytes()[i] - b'A') as usize] >= needed {
                    h[(s.as_bytes()[i] - b'A') as usize] -= needed;
                    let mut s2 = String::from(&s[..i]);
                    s2.push_str(&s[j..]);
                    ans = std::cmp::min(ans, needed + dfs(&s2, h, true));
                    ans = std::cmp::min(ans, needed + dfs(&s2, h, false));
                    h[(s.as_bytes()[i] - b'A') as usize] += needed;
                }
                i = j;
            }
            ans
        }
        let mut hand_count = vec![0; 26];
        for i in 0..hand.len() {
            hand_count[(hand.as_bytes()[i] - b'A') as usize] += 1;
        }
        board.push('#');
        let step = dfs(&board, &mut hand_count, true);
        if step == MAX_COUNT {
            -1
        } else {
            step
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min_step() {
        assert_eq!(
            Solution::find_min_step("WRRBBW".to_owned(), "RB".to_owned()),
            -1
        );
    }

    #[test]
    fn test_find_min_step_02() {
        assert_eq!(
            Solution::find_min_step("WWRRBBWW".to_owned(), "WRBRW".to_owned()),
            2
        );
    }

    #[test]
    fn test_find_min_step_03() {
        assert_eq!(
            Solution::find_min_step("G".to_owned(), "GGGGG".to_owned()),
            2
        );
    }

    #[test]
    fn test_find_min_step_04() {
        assert_eq!(
            Solution::find_min_step("RBYYBBRRB".to_owned(), "YRBGB".to_owned()),
            3
        );
    }

    #[test]
    fn test_find_min_step_05() {
        assert_eq!(
            Solution::find_min_step("RRWWRRBBRR".to_owned(), "RRWWRRBBRR".to_owned()),
            2
        );
    }
}