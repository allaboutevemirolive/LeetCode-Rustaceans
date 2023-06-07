// https://leetcode.com/problems/maximum-score-words-formed-by-letters/solutions/3443996/rust-knapsack-problem-dfs/
impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let num_words = words.len();

        // Convert each word in words into [u8; 26] array of letter counts
        let word_arrs: Vec<[u8; 26]> = words
            .iter()
            .map(|word| {
                word
                    .as_bytes()
                    .iter()
                    .fold([0; 26], |mut arr, &byte| {
                        arr[(byte-b'a') as usize] += 1;
                        arr
                    })
            })
            .collect();

        // Take inventory of the letters we have, convert from char to byte
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

        // Do a depth-first search of possible words
        // Use a hashmap to keep track of what word combinations we've already visited
        let mut visited = std::collections::HashMap::<u16, i32>::new();
        visited.insert(0, 0);

        // Start our search at "no words", full letter inventory, no score
        let mut stack: Vec<(u16, [u8; 26], i32)> = Vec::new();
        stack.push((0, inventory, 0));

        // Do the search until completed
        while let Some((used, available, curr_score)) = stack.pop() {
            // Try adding each word
            for word in 0..num_words {
                if (used >> word) & 1 == 1 {
                    // Word is already being used, can't add twice
                    continue;
                }

                let new_used = used | (1 << word);
                if visited.contains_key(&new_used) {
                    // Already visited this combination of words
                    continue;
                }

                // Check to see if this new word combination is possible and reduce inventory accordingly
                let mut possible = true;
                let mut new_available = available.clone();
                let mut new_score = curr_score;

                for letter in 0..26 {
                    if word_arrs[word][letter] > available[letter] {
                        // Don't have enough letters left to add this word
                        possible = false;
                        break;
                    } else {
                        // Subtract the word's letter costs from available inventory
                        new_available[letter] -= word_arrs[word][letter];
                        // Update score to reflect added letter
                        new_score += (word_arrs[word][letter] as i32) * score[letter];
                    }
                }

                if !possible {
                    continue;
                }

                // Add the new word combination to the stack
                visited.insert(new_used, new_score);
                stack.push((new_used, new_available, new_score));
            }
        }

        // Take the best score out of all visited combinations
        // If no word combinations are possible, score is 0
        *visited.values().max().unwrap_or(&0)
    }
}