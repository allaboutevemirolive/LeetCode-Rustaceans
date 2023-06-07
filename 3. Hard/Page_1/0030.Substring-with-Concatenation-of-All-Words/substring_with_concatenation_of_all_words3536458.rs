// https://leetcode.com/problems/substring-with-concatenation-of-all-words/solutions/3536458/rust-easy-to-understand/
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if words.is_empty() || s.is_empty() {
            return vec![];
        }

        let word_len = words[0].len();
        let s_len = s.len();
        if s_len < words.len() * word_len {
            return vec![];
        }

        let check = words.iter().fold(String::new(), |mut res, i| { res.push_str(i); res});
        if check.eq(&s) {
            return vec![0];
        }

        let mut res = vec![];
        let mut idx = 0;

        'out: while idx + word_len <= s_len && s_len - idx >= words.len() * word_len {
            for (in_idx, ele) in words.iter().enumerate() {
                if &s[idx..idx + word_len] == ele {
                    let mut new_words = words.clone();
                    new_words.remove(in_idx);

                    let mut index = idx + word_len;

                    'empty: while !new_words.is_empty() && index + word_len <= s_len {
                        for (key, val) in new_words.iter().enumerate() {
                            if val == &s[index..index + word_len] {
                                new_words.remove(key);
                                index += word_len;
                                continue 'empty;
                            }
                        }

                        idx += 1;
                        continue 'out;
                    }

                    if new_words.is_empty() {
                        res.push(idx as i32);
                    }
                    
                    idx += 1;
                    continue 'out;
                }
            }

            idx += 1;
        }

        return res;
    }
}