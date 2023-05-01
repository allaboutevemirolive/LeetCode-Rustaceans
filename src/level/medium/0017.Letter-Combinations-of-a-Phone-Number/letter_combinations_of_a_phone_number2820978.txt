// https://leetcode.com/problems/letter-combinations-of-a-phone-number/solutions/2820978/rust-permutations/
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        
        use std::collections::HashMap;
        let lookup = HashMap::from([
            ('2', "abc".chars().collect::<Vec<_>>()),
            ('3', "def".chars().collect::<Vec<_>>()),
            ('4', "ghi".chars().collect::<Vec<_>>()),
            ('5', "jkl".chars().collect::<Vec<_>>()),
            ('6', "mno".chars().collect::<Vec<_>>()),
            ('7', "pqrs".chars().collect::<Vec<_>>()),
            ('8', "tuv".chars().collect::<Vec<_>>()),
            ('9', "wxyz".chars().collect::<Vec<_>>()),
        ]);

        fn permute(
            digits: &Vec<char>,
            permutation: &mut String,
            pressed: usize,
            res: &mut Vec<String>,
            lookup: &HashMap<char, Vec<char>>,
        ) {
            if permutation.len() == digits.len() {
                res.push(permutation.to_owned());
                return;
            }

            let letters = lookup.get(&digits[pressed]).unwrap();
            for ch in letters {
                permutation.push(*ch);
                permute(digits, permutation, pressed + 1, res, lookup);
                permutation.pop();
            }
        }

        let mut res = vec![];
        permute(
            &digits.chars().collect::<Vec<_>>(),
            &mut String::new(),
            0,
            &mut res,
            &lookup,
        );
        res
    }
}