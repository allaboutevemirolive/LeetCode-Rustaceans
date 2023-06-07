// https://leetcode.com/problems/count-all-valid-pickup-and-delivery-options/solutions/1655396/o-n-runtime-o-1-space-simple-explanation-100-100-6-lines/
    pub fn count_orders(n: i32) -> i32 {
        let mut last: u64 = 1;
        for i in 2u64..= (n as u64) {
            let gaps: u64 = 2 * i  - 1; 
            last *= (gaps + 1) * gaps / 2;
            last %= (1000000000u64 + 7u64); // just for overflow control as dictated by the prompt
        }
        last as i32
    }