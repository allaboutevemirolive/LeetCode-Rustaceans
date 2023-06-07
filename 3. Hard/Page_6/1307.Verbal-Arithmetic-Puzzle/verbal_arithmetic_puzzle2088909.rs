// https://leetcode.com/problems/verbal-arithmetic-puzzle/solutions/2088909/faster-than-100-of-rust-solutions-best-explanation/
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn is_solvable(words: Vec<String>, result: String) -> bool {
        let mut equation: Vec<Vec<char>> = Vec::new();
        for w in words {
            if w.len() > result.len() {
                return false;
            }
            equation.push(w.chars().rev().collect());
        }
        let solution = &mut HashMap::new();
        let ans: Vec<char> = result.chars().rev().collect();
        if Self::can_solve(&equation, &ans, 0, 0, 0, solution) {
            println!("{:?}", solution);
            true
        } else {
            false
        }
    }
    
    fn can_solve(
        equation: &[Vec<char>],
        result: &[char],
        row: usize,
        col: usize,
        carry: u32,
        solution: &mut HashMap<char, u8>,
    ) -> bool {
        let addend = row < equation.len();
        if addend && col >= equation[row].len() {
            // No leading zero for multicharacter words
            let word = &equation[row];
            if solution[&word[word.len() - 1]] == 0 && word.len() > 1 {
                return false;
            }
            return Self::can_solve(equation, result, row + 1, col, carry, solution);
        }
        if col == result.len() && !addend {
            return carry == 0 && (col == 1 || solution[&result[col - 1]] > 0);
        }

        let digit = if addend {
            equation[row][col]
        } else {
            result[col]
        };
        let assigned = solution.contains_key(&digit);

        if addend {
            if assigned {
                Self::can_solve(
                    equation,
                    result,
                    row + 1,
                    col,
                    carry + (solution[&digit] as u32),
                    solution,
                )
            } else {
                let used: HashSet<&u8> = HashSet::from_iter(solution.values());
                let unused: Vec<u8> = (0..=9).filter(|x| !used.contains(x)).collect();
                for i in unused {
                    solution.insert(digit, i);
                    if Self::can_solve(equation, result, row + 1, col, carry + (i as u32), solution) {
                        return true;
                    }
                    solution.remove(&digit);
                }
                false
            }
        } else {
            let sum_digit = (carry % 10) as u8;
            if assigned {
                (solution[&digit] == sum_digit)
                    && Self::can_solve(equation, result, 0, col + 1, carry / 10, solution)
            } else {
                let used = solution.values().any(|&x| x == sum_digit);
                if used {
                    return false;
                }
                solution.insert(digit, sum_digit);
                if Self::can_solve(equation, result, 0, col + 1, carry / 10, solution) {
                    return true;
                }
                solution.remove(&digit);
                false
            }
        }
    }
}