// https://leetcode.com/problems/stream-of-characters/solutions/1610905/rust-sorted-array-binary-search-solution/
struct StreamChecker {
  dict: Vec<Vec<char>>,
  query: Vec<char>,
}

// This function is in the standard library since 1.52:
// https://doc.rust-lang.org/std/primitive.slice.html#method.partition_point
// but LeetCode Rust version is lacking. Reimplemented.
fn partition_point<F>(words: &[Vec<char>], mut f: F) -> usize where F: FnMut(&Vec<char>) -> bool {
  let mut lo = 0;
  let mut hi = words.len();
  while lo < hi {
    let mi = lo + (hi - lo)/2;
    if f(&words[mi]) {
      lo = mi + 1;
    } else {
      hi = mi;
    }
  }
  lo
}

fn prefix_exists(words: &[Vec<char>], char_i: usize, query: &[char]) -> bool {
  let last_c = query[query.len() - 1 - char_i];
  let lo = partition_point(words, |x| x[char_i] < last_c);
  let hi = partition_point(words, |x| x[char_i] <= last_c);
  assert!(lo <= hi);
  if lo == hi {
    // No words matching the prefix
    false
  } else if words[lo..hi].iter().any(|w| w.len() <= char_i + 1) { // This could be done in sub-linear time, but seems to be good enough
    // Prefix matches some words
    true
  } else if query.len() <= char_i + 1 {
    // Query is too short
    false
  } else {
    // So far so good, check the rest of the prefix
    prefix_exists(&words[lo..hi], char_i + 1, query)
  }
}

impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
      let mut dict = words.iter().map(|s| s.chars().rev().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
      dict.sort();
      StreamChecker{dict, query: vec![]}
    }
    
    fn query(&mut self, letter: char) -> bool {
      self.query.push(letter);
      prefix_exists(&self.dict, 0, &self.query)
    }
}