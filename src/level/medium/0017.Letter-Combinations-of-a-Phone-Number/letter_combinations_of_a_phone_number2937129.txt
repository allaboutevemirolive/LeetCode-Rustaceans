// https://leetcode.com/problems/letter-combinations-of-a-phone-number/solutions/2937129/rust-solution/
use std::collections::HashMap;

impl Solution {
    fn get_letters(digits: &str) -> Vec<Vec<&str>> {
        let letters: HashMap<_, _> = [
            ('2', vec!["a", "b", "c"]),
            ('3', vec!["d", "e", "f"]),
            ('4', vec!["g", "h", "i"]),
            ('5', vec!["j", "k", "l"]),
            ('6', vec!["m", "n", "o"]),
            ('7', vec!["p", "q", "r", "s"]),
            ('8', vec!["t", "u", "v"]),
            ('9', vec!["w", "x", "y", "z"]),
        ]
        .into();
        digits
            .chars()
            .map(|c| letters.get(&c).unwrap().clone())
            .collect()
    }

    fn combine(
        remaining: usize,
        letters_group: &[Vec<&str>],
        combination: String,
        combinations: &mut Vec<String>,
    ) {
        if remaining == 0 {
            if !combination.is_empty() {
                combinations.push(combination.clone());
                return;
            }
        } else {
            for i in 0..letters_group.len() {
                for j in 0..letters_group[i].len() {
                    let mut combination = combination.clone();
                    combination.push_str(letters_group[i][j]);
                    Self::combine(
                        remaining - 1,
                        &letters_group[i + 1..],
                        combination,
                        combinations,
                    );
                }
            }
        }
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        let letters = Self::get_letters(&digits);
        if digits.len() == 1 {
            return letters
                .into_iter()
                .flat_map(|l| l.into_iter().map(String::from).collect::<Vec<_>>())
                .collect();
        }
        let mut combinations = Vec::new();
        Self::combine(letters.len(), &letters, String::new(), &mut combinations);
        combinations
    }
}
