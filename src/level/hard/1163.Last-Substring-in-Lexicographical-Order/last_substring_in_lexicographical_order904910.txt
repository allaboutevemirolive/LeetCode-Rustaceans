// https://leetcode.com/problems/last-substring-in-lexicographical-order/solutions/904910/rust-o-n-2-solution-4-ms-how-to-improve/
impl Solution {
    pub fn last_substring(s: String) -> String {
        if s.is_empty(){
            return s;
        }
		// Don't bother with unicode.
        let bytes = s.as_bytes();
        let max_byte = bytes.iter().copied().max().unwrap();
        if bytes.iter().all(|&b|b==max_byte){
            return s;
        }
		// Positions with possible starts of solution
        let mut positions: Vec<usize> = bytes
            .iter()
            .copied()
            .enumerate()
            .filter(|&(_, b)|b==max_byte)
            .map(|(i, _)|i)
            .collect();
        
		// Now filter candidates
        let mut current_forward = 0;
		// loop over all found positions
		// Since positions is O(n) and iteration is O(n)
		// O(n^2) finally
        while positions.len() > 1 {
            current_forward += 1;
			// Here O(n) lookup for max char at some jump forward over positions
            let max_byte = positions
                    .iter()
                    .copied()
					// Here I actually filter if some jump too big (over string end)
                    .flat_map(|i|bytes.get(i+current_forward))
					// Here look for max char
                    .copied()
                    .max()
                    .expect("At least 1 item");
			// Now remove all non max `current_forward` len candidates
			// O(n)
            positions.retain(|&i|bytes.get(i+current_forward) == Some(&max_byte));
        }
        if positions[0] == 0{
            return s;
        }
        std::str::from_utf8(&bytes[positions[0]..]).expect("Expected ASCII").to_string()
    }
}