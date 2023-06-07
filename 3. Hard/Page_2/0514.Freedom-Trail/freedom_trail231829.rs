// https://leetcode.com/problems/freedom-trail/solutions/231829/annotated-dp-solution-using-rust/
impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let mut index = vec![Vec::new(); 26]; // indices of each ring character
        for (idx, char) in ring.chars().enumerate() {
            index[(char as u8 - 'a' as u8) as usize].push(idx);
        }
        // d[i][j] = min steps to rotate key[j..] with ring pointing at index i
        let mut d = vec![vec![0; key.len()+1]; ring.len()];
        for k_pos in (0..key.len()).rev() {
            for r_pos in (0..ring.len()) {
                let mut min_so_far = std::i32::MAX;
                for r_idx in &index[(key.as_bytes()[k_pos] - 'a' as u8) as usize] {
                    // find the clockwise rotation from current position r_pos to
                    // position r_idx pointing at the desired character key[k_pos]
                    let distance = (r_pos as i32 - *r_idx as i32).abs();
                    // check if counter-clockwise rotation is shorter
                    let min_distance = std::cmp::min(distance, ring.len() as i32 - distance);
                    // relax min_so_far in case rotating to r_idx is beneficial
                    min_so_far = std::cmp::min(min_so_far, min_distance + d[*r_idx][k_pos+1]) 
                }
                d[r_pos][k_pos] = min_so_far + 1; // steps to rotate + pressing the center button
            }
        }
        d[0][0] // min steps to rotate k[0..] with ring at its initial position 0
    }
}