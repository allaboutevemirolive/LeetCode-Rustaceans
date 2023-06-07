// https://leetcode.com/problems/stickers-to-spell-word/solutions/495587/rust-124ms-2-1mb-100/
// https://leetcode.com/problems/stickers-to-spell-word/discuss/108333/Rewrite-of-contest-winner's-solution

    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        let n = target.len();
        let n2 = 1 << n;
        let mut dp = vec![u32::max_value() as u32; n2];
        dp[0] = 0;
        for i in 0..n2 {
            if dp[i] == u32::max_value() { continue; }
            for s in &stickers {
                let mut now = i;
                for c in s.chars() {
                    for r in 0..n {
                        if target[r..].chars().next().unwrap() == c && !((now >> r) & 1 != 0) {
                            now |= 1 << r;
                            break;
                        }
                    }
                }
                dp[now] = std::cmp::min(dp[now], dp[i] + 1);
            }
        }
        dp[(n2 - 1) as usize] as i32
    }