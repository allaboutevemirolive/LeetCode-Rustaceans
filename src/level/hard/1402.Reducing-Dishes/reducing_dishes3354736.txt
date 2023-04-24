// https://leetcode.com/problems/reducing-dishes/solutions/3354736/rust-solution/
impl Solution {
    pub fn max_satisfaction(mut s: Vec<i32>) -> i32 {
        s.sort_unstable();
        let z = s.binary_search(&0).unwrap_or_else(|v|v);
        let r_tick = s.iter().skip(z).sum::<i32>();
        let mut l_tick = s.iter().take(z).sum::<i32>();
         s.into_iter().skip_while(|&v| {
            if -r_tick < l_tick {
                return false
            }
            l_tick -= v;
            true
        }).fold((0,1), |(res, mult), val| {
            (res + val*mult ,mult+1)
        }).0
    }
}