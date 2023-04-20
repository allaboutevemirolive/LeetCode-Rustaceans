// https://leetcode.com/problems/orderly-queue/solutions/2784459/rust-zero-addtional-space/
impl Solution {
    pub fn orderly_queue(mut s: String, k: i32) -> String {
        let b = unsafe { s.as_bytes_mut() };
        if k > 1 {
            b.sort_unstable();
            return s;
        }
 
        macro_rules! rot {
            [$n:expr] => (b.iter().cycle().skip($n).take(b.len()))
        }
        let mut n = 0;
        for i in 1..b.len() {
            if rot![i].lt(rot![n]) {
                n = i;
            }
        }
        b.rotate_left(n);
        s
    }
}