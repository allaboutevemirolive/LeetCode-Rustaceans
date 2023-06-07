// https://leetcode.com/problems/sliding-window-median/solutions/1942951/rust-5ms-100/
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let k = k as usize;
        
        let mut sorted = Vec::from(&nums[..k]);
        sorted.sort_unstable();
        
        let mut out = Vec::new();
        
        for i in 0..nums.len()-k {
            out.push(median(&sorted));
            
            let removal_val = nums[i];
            let add_val = nums[i+k];
            
            sorted.remove(
                sorted.binary_search(&removal_val).unwrap()
            );
            
            match sorted.binary_search(&add_val) {
                Ok(i)  => sorted.insert(i, add_val),
                Err(i) => sorted.insert(i, add_val),
            };
        }
        
        out.push(median(&sorted));
        
        out
    }
}

fn median(v: &[i32]) -> f64 {
    let L = v.len();

    if L % 2 == 1 {
        v[L/2] as f64
    } else {
        (v[L/2-1] as f64 + v[L/2] as f64) / 2.0
    }
}