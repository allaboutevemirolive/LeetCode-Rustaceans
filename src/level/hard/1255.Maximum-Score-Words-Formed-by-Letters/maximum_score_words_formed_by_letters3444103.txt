// https://leetcode.com/problems/maximum-score-words-formed-by-letters/solutions/3444103/rust-heapless-dfs-0ms-and-beats-100-memory/
impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let inventory = {
            // Used for encoding as UTF-8
            let mut buf: [u8; 4] = [0, 0, 0, 0];
            let mut inventory = [0; 26];
            
            for letter in letters.into_iter() {
                inventory[(letter.encode_utf8(&mut buf).as_bytes().get(0).unwrap() - b'a') as usize] += 1;
            }

            // Return from block, make immutable
            inventory
        };

        Solution::recursive(&words, inventory, &score)
    }

    fn recursive(words: &[String], inventory: [u8; 26], score: &[i32]) -> i32 {
        // Base case
        if words.len() == 0 {
            return 0;
        }

        // What's the best score we can get without the last word in words?
        let best_without = Solution::recursive(&words[0..words.len()-1], inventory, score);

        // What's the best score we can get with the last word in words?
        let word = words[words.len()-1].as_str();
        let mut word_score = 0;
        let mut new_inventory = inventory;

        for &byte in word.as_bytes().iter() {            
            let letter = (byte - b'a') as usize;
            if new_inventory[letter] == 0 {
                // We don't have enough letters to take this word
                return best_without;
            }
            new_inventory[letter] -= 1;
            word_score += score[letter];
        }

        best_without.max(word_score + Solution::recursive(&words[0..words.len()-1], new_inventory, score))
    }
}