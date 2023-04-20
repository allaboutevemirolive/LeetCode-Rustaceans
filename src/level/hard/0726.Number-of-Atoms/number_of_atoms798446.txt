// https://leetcode.com/problems/number-of-atoms/solutions/798446/rust-ll-1-parser/
use std::collections::BTreeMap;
impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        let mut idx = 0;
        let formula: Vec<char> = formula.chars().collect();
        let map = Self::parse(&formula, &mut idx);
        map.iter().fold(String::new(), |ret, (ch, ct)| {
            if *ct > 1 {
                format!("{}{}{}", ret, ch, ct)
            } else {
                format!("{}{}", ret, ch)
            }
        })
    }
    fn parse(formula: &Vec<char>, idx: &mut usize) -> BTreeMap<String, i32> {
        let mut ret = BTreeMap::<String, i32>::new();
        while *idx < formula.len() {
            if formula[*idx] == '(' {
                *idx += 1;
                let temp = Self::parse(formula, idx);
                temp.iter().for_each(|(k, v)| {
                    *ret.entry(k.to_owned()).or_default() += v;
                });
            } else if formula[*idx] == ')' {
                *idx += 1;
                let count = Self::parse_number(formula, idx);
                ret.values_mut().for_each(|v| {
                    *v *= count;
                });
                return ret;
            } else {
                let name = Self::parse_name(formula, idx);
                let count = Self::parse_number(formula, idx);
                *ret.entry(name).or_default() += count;
            }
        }
        ret
    }
    fn parse_name(formula: &Vec<char>, idx: &mut usize) -> String {
        let mut ret = String::new();
        let mut first = true;
        while *idx < formula.len() && formula[*idx].is_alphabetic() {
            if !first && formula[*idx].is_uppercase() {
                return ret;
            }
            first = false;
            ret.push(formula[*idx]);
            *idx += 1;
        }
        ret
    }
    fn parse_number(formula: &Vec<char>, idx: &mut usize) -> i32 {
        let mut ret = 0;
        while *idx < formula.len() && formula[*idx].is_numeric() {
            ret *= 10;
            ret += formula[*idx].to_digit(10).unwrap() as i32;
            *idx += 1;
        }
        if ret == 0 {
            1
        } else {
            ret
        }
    }
}