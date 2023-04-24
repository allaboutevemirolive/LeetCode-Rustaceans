// https://leetcode.com/problems/number-of-valid-words-for-each-puzzle/solutions/1529729/rust-using-simple-bitwise-and/
pub fn find_num_of_valid_words<W: AsRef<str>>(words: Vec<W>, puzzles: Vec<W>) -> Vec<i32> {
    let mut words = words.iter().map(|w| word_to_int(w)).collect::<Vec<_>>();
    let mut puzzles = puzzles
        .iter()
        .map(|w| {
            let as_int = word_to_int(w);
            let first_ch = first_char_to_mask(w);
            (as_int, first_ch)
        })
        .collect::<Vec<_>>();

    let mut result = Vec::with_capacity(puzzles.len());
    for &(puzzle, first_ch) in puzzles.iter() {
        let mut counter = 0;
        for &word in words.iter() {
            if (word & puzzle == word) & (word & first_ch == first_ch) {
                counter += 1;
            }
        }
        result.push(counter);
    }
    result
}

fn word_to_int<W: AsRef<str>>(word: W) -> u32 {
    let word = word.as_ref().as_bytes();
    let mut bits = 0;

    for &ch in word {
        bits |= 1 << ch - b'a';
    }

    bits
}

fn first_char_to_mask<W: AsRef<str>>(word: W) -> u32 {
    let word = word.as_ref().as_bytes();
    1 << word[0] - b'a'
}