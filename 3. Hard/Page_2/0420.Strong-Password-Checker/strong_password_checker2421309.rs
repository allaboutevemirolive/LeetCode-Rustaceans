// https://leetcode.com/problems/strong-password-checker/solutions/2421309/0ms-rust-solution-with-explanatory-comments/
impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        // This is the change needed in the length of the password
        // to fit in the declared criterion.
        let mut char_change = loop {
           let len = password.len() as i32;
            if len < 6 {
                break 6-len;
            }
            if len > 20 {
                break 20-len;
            }
            break 0;
        }; 
        
        
        // These are 1 when we need to add the char class, 0 otherwise.
        let should_add_lowercase = password.chars().fold(1, |acc, chr| if acc==0 {0} else {if chr.is_ascii_lowercase() {0} else {1}});
        let should_add_uppercase = password.chars().fold(1, |acc, chr| if acc==0 {0} else {if chr.is_ascii_uppercase() {0} else {1}});
        let should_add_digit = password.chars().fold(1, |acc, chr| if acc==0 {0} else {if chr.is_ascii_digit() {0} else {1}});

        // We create an array of tuples that represent consecutive characters of length
        // more than 3.
        let mut consecutives = {
            let mut consecutives = Vec::new();
            let mut consecutive: (char, u8) = (' ', 0);

            for chr in password.chars() {
                if consecutive.0 == chr {
                    consecutive.1 += 1;
                }
                else {
                    if consecutive.1 > 2 {
                        consecutives.push(consecutive);
                    }
                    consecutive = (chr, 1);
                }
            }
            if consecutive.1 > 2 {
                consecutives.push(consecutive);
            }
            consecutives
        };
        
        if char_change > 0 {
            // Missing chars can be added and consecutive problems can be solved in char_change moves if char_change >= 3.
            // If it's 2, missings might be bigger but consecutives is still within this limit.
            // If it's 1, there might be a consecutive of 5 which needs 2 changes but if that's the case the addition of 3 missings' variables will be bigger than or equal to 2.
            return (should_add_lowercase + should_add_uppercase + should_add_digit).max(char_change);
        }
        
        let mut removed = 0;
        
        if char_change < 0 {
		    // Aim here is to move toward making every consecutive be 2 mod 3.
            // This is optimal because at 2 mod 3 we've used the least amount of removals to 
            // decrease the most amount of operations needed in the future for each consecutive.

            // We first remove 3-divisible consecutives
            for c in consecutives.iter_mut() {
                if char_change >= 0 {
                    break;
                }
                
                if c.1 % 3 == 0 {
                    c.1 -= 1;
                    char_change += 1;
                    removed += 1;
                }
            }
            
            consecutives = consecutives.into_iter().filter(|&(_, num)| num >= 3).collect();
            
            // We then remove mod 1 consecutives
            for c in consecutives.iter_mut() {
                if char_change >= 0 {
                    break;
                }
                
                if c.1 % 3 == 1 {
                    // Make sure not to subtract more than needed.
                    let subtract = (2).min(char_change.abs() as u8);
                    c.1 -= subtract;
                    char_change += subtract as i32;
                    removed += subtract as i32;
                }
            }
            
            consecutives = consecutives.into_iter().filter(|&(_, num)| num >= 3).collect();
            
            // Now we greedily remove as many as we can to get rid of consecutives.
            // It will not matter which ones we remove from as all of them are 2 mod 3.
            for c in consecutives.iter_mut() {
                if char_change >= 0 {
                    break;
                }
                
                // We only need to subtract to make c.1 less than 3.
                let subtract = (c.1-2).min(char_change.abs() as u8);
                
                c.1 -= subtract;
                char_change += subtract as i32;
                removed += subtract as i32;
            }
            
            consecutives = consecutives.into_iter().filter(|&(_, num)| num >= 3).collect();
        }
        
        // Now we have two cases:
        // 1. # of chars to be removed was bigger than total consecutive chars
        // 2. total consective chars was bigger than # of chars to be removed
        
        // 1 => there are no longer any ops to be done on consecutives but only chars to be removed.
        // 2 => there are no longer any chars to be removed, just ops to do on consecutives.
        
        if char_change != 0 {
            // Case 1
            // We've only removed `removed` many elements. we have to remove char_change.abs() many.
            // We also have to add missing char classes.
            
            return removed + char_change.abs() + (should_add_lowercase + should_add_uppercase + should_add_digit);
        }
        else {
            // Case 2
            
            // Adding characters will only isolate 2 characters from the consecutive group
            // but changing a character will decrease the count by 3!
            
            // So at this point we only change characters.
            let mut changed = 0;
            
            for c in consecutives.iter_mut() {
                // we only need to change c.1/3 chars.
                changed += (c.1/3) as i32;
            }
            
            // if we didn't have enough characters to change, we will have to change characters
            // to make sure missing char classes are included.
            // (we say change missing char classes because adding might cause us to exceed the length limit)
            return removed + changed.max(should_add_lowercase+should_add_uppercase+should_add_digit);
        }
    }
}