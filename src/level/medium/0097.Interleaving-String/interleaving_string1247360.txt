// https://leetcode.com/problems/interleaving-string/solutions/1247360/rust-o-s2-length-additional-memory-space/
impl Solution {
  pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {    
    let b = s2.as_bytes();
    let c = s3.as_bytes();
    let m = b.len();
    if m + s1.bytes().len() != c.len() { return false; }
    let mut x = vec![ false; m + 1];
    x[0] = true;
    for (i, z) in s1.bytes().chain(std::iter::once(0)).enumerate() {
      for j in 0 .. m {
        if x[j] {
          let k = i + j;
          if b[j] == c[k] {
            x[j+1] = true;
          }
        }
      }
      if z == 0 { break; }
      let mut y = vec![ false; m + 1];
      for j in 0 ..= m {
        if x[j] {
          let k = i + j;
          if z == c[k] {
            y[j] = true;
          }          
        }        
      }
      x = y;
    } 
    x[m]
  }
}

#[test]
fn unittests() {
  let check = |s1: &str, s2: &str, s3: &str, b| {
    println!("{} {} {}", s1, s2, s3);
    assert_eq!(Solution::is_interleave(s1.to_string(), s2.to_string(), s3.to_string()), b);
  };
  check("a", "", "aa", false);
  check("a", "b", "ba", true);
  check("a", "b", "ab", true);
  check("aabcc", "dbbca", "aadbbcbcac", true);
  check("aabcc", "dbbca", "aadbbbaccc", false);
  check("", "", "", true);
}