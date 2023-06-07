// https://leetcode.com/problems/parsing-a-boolean-expression/solutions/323562/rust-clean-solution-using-recursion/
use std::str::Chars;

impl Solution {
    pub fn parse_next(iter: &mut Chars) -> bool {
        match iter.next() {
            Some('t') => true,
            Some('f') => false,
            Some('&') => Solution::parse_and(iter),
            Some('|') => Solution::parse_or(iter),
            Some('!') => Solution::parse_not(iter),
            _ => unreachable!()
        }
    }

    pub fn parse_not(iter: &mut Chars) -> bool {
        let mut result = true;
        loop {
            match iter.next() {
                Some(')') => return !result,
                Some('(') => result = Solution::parse_next(iter),
                _ => unreachable!()
            }
        }
    }

    pub fn parse_and(iter: &mut Chars) -> bool {
        let mut result = true;
        loop {
            match iter.next() {
                Some(')') => return result,
                Some('(') => result = Solution::parse_next(iter),
                Some(',') => result = Solution::parse_next(iter) && result,
                _ => unreachable!()
            }
        }
    }
    
    pub fn parse_or(iter: &mut Chars) -> bool {
        let mut result = false;
        loop {
            match iter.next() {
                Some(')') => return result,
                Some('(') => result = Solution::parse_next(iter),
                Some(',') => result = Solution::parse_next(iter) || result,
                _ => unreachable!()
            }
        }
    }

    pub fn parse_bool_expr(expression: String) -> bool {
        let mut iter = expression.chars();
        Solution::parse_next(&mut iter)
    }
}