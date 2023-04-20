// https://leetcode.com/problems/decode-ways-ii/solutions/1087223/rust-12ms-3-8-mb/
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        // Easier to work with a vec than directly with the string
        let characters: Vec<char> = s.chars().collect();
        if characters.first().unwrap() == &'0' {
            return 0;
        }
        
        // ways[i] is the number of ways to decode the substring ending at index i - 1
        // Note: u64 to avoid overflow in addition prior to modulus 
        let mut ways: Vec<u64> = vec![0; s.len() + 1];
        ways[0] = 1;
        
        // Special case
        ways[1] = if characters[0] == '*' { 9 } else { 1 };
        
        
        for i in 2..=characters.len() {
            let single_char = characters[i - 1];
            match &single_char {
                '*' => { ways[i] += 9 * ways[i - 1]; }
                _ => {
                    let single_character = single_char.to_digit(10).unwrap();
                    if single_character >= 1 && single_character <= 9 {
                        ways[i] += ways[i - 1];
                    }
                }
            }
            
            let double_chars = (characters[i-2], characters[i-1]);
            match double_chars {
                // 10..=26
                ('*', '*') => { ways[i] += 15 * ways[i-2]; },
                ('*', char_2) => {
                    let digit_2 = char_2.to_digit(10).unwrap();
                    
                    // 17 | 18 | 19
                    if digit_2 > 6 {
                        ways[i] += ways[i-2];
                    } 
                    // e.g. 11, 21
                    else {
                        ways[i] += 2 * ways[i-2];
                    }
                },
                // 11,12,13,14,15,16,17,18,19
                ('1', '*') => {
                    ways[i] += 9 * ways[i - 2]
                }
                // 21,22,23,24,25,26
                ('2', '*') => {
                    ways[i] += 6 * ways[i - 2]
                }
                // e.g. 3*
                (_, '*') => {},
                // e.g. 45
                (_, _) => {
                    let double_character = s[i - 2..i].parse::<i32>().unwrap();


                    if double_character >= 10 && double_character <= 26 {
                        ways[i] += ways[i - 2];
                    }
                }
            }
            
            // We take the modulus at each step to prevent overflow. Since all recurrence operations
            // are additive, this is transitive (we can do it at each step and have the same result
            // as doing it only at the end)
            ways[i] = ways[i].rem_euclid(1000000007)
        }
        ways[s.len()] as i32
    }
}