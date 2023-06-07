// https://leetcode.com/problems/smallest-good-base/solutions/495430/rust-0ms-2-4m-100/
pub fn smallest_good_base(n: String) -> String {
    let num = n.parse::<i128>().unwrap();
    let x = 1i128;
    for p in (1..64).rev() {
        if x << p < num {
            let k = helper(num, p);
            if k != -1 { return k.to_string(); }
        }
    }
    return (num - 1).to_string();

    fn helper(num: i128, p: i32) -> i128 {
        let mut l = 1;
        let mut r = ((num as f64).powf(1.0 / p as f64) + 1.0).trunc() as i128;
        while l < r {
            let mid = l + (r - l) / 2;

            let mut sum = 0i128;
            let mut cur = 1;
            for i in 0..p + 1 {
                sum += cur;
                cur *= mid;
            }
            if sum == num { return mid; } else if sum > num { r = mid; } else { l = mid + 1 };
        }
        -1
    }
}