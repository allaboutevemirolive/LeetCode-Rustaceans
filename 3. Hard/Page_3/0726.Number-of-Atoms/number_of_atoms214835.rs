// https://leetcode.com/problems/number-of-atoms/solutions/214835/0ms-parser-in-rust/
use std::collections::HashMap;

impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        let mut counts: HashMap<&str, u32> = HashMap::new();
        let mut stack: Vec<HashMap<&str, u32>> = Vec::new();
        let mut formula: &str = &formula;
        while !formula.is_empty() {
            if formula.starts_with("(") {
                formula = formula.get(1..).unwrap(); // consume (
                stack.push(counts);
                counts = HashMap::new();
            } else if formula.starts_with(")") {
                formula = formula.get(1..).unwrap(); // consume )
                let count_len = formula.chars().take_while(|c| c.is_digit(10)).count();
                let count = if count_len > 0 { formula[..count_len].parse().unwrap() } else { 1 };
                formula = formula.get(count_len..).unwrap(); // consume count
                for (_, val) in counts.iter_mut() {
                    *val *= count;
                }
                counts = match stack.pop() {
                    Some(mut top) => {
                        for (key, val) in counts.iter() {
                            *top.entry(key).or_insert(0) += *val;
                        }
                        top
                    },
                    None => counts,
                };
            } else {
                // capital letter followed by lowercase letters
                let name_len = 1 + formula.chars().skip(1).take_while(|c| c.is_lowercase()).count();
                let name = &formula[..name_len];
                formula = formula.get(name_len..).unwrap(); // consume name
                let count_len = formula.chars().take_while(|c| c.is_digit(10)).count();
                let count = if count_len > 0 { formula[..count_len].parse().unwrap() } else { 1 };
                formula = formula.get(count_len..).unwrap(); // consume count
                *counts.entry(name).or_insert(0) += count;
            }
        }
        let mut elements: Vec<_> = counts.iter().collect();
        elements.sort_unstable();
        let mut output: String = String::new();
        for (name, count) in elements {
            output.push_str(&name);
            if *count > 1 {
                output.push_str(&count.to_string());
            }
        }
        output
    }
}