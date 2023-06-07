// https://leetcode.com/problems/orderly-queue/solutions/1446452/rust-o-n-0ms-suffix-array-solution/
impl Solution {
    // @robertkingnz
    pub fn orderly_queue(s: String, k: i32) -> String {
        if k >= 2 {
            let mut v: Vec<char> = s.chars().collect();
            v.sort();
            return v.iter().map(|x| *x).collect();
        }
        let sa = SuffixTable::new(format!("{}{}",s,s)); // O(N)
        let t = sa.table(); 
		// table[i] gives the suffix in order, like in https://en.wikipedia.org/wiki/Suffix_array so we just want the first suffix we find that's in the first half.
        for j in 0..t.len() {
            let i = t[j] as usize;
            if i < s.len() {
                let mut it = sa.suffix(j).chars();
                return (0..s.len()).map(|_| it.next().unwrap()).collect();
            }
        }
        unreachable!();
    }
}