// https://leetcode.com/problems/shortest-common-supersequence/solutions/494586/rust-4ms-9-7mb-100/
pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
    let m = str1.len();
    let n = str2.len();
    if m == 0 { return str2; }
    if n == 0 { return str1; }
    let s1 = str1.chars().collect::<Vec<char>>(); // should void for memory
    let s2 = str2.chars().collect::<Vec<char>>(); // should void for memory

    let mut dp = vec![vec![0; n + 1]; m + 1];
    for i in 0..m + 1 {
        for j in 0..n + 1 {
            if i == 0 { dp[i][j] = j; } else if j == 0 { dp[i][j] = i; } else if s1[i - 1] == s2[j - 1] {
                dp[i][j] = 1 + dp[i - 1][j - 1];
            } else { dp[i][j] = 1 + std::cmp::min(dp[i - 1][j], dp[i][j - 1]); }
        }
    }
    let mut l = dp[m][n];
    let mut ans = vec![' '; l];
    let mut i = m;
    let mut j = n;
    while i > 0 && j > 0 {
        l -= 1;
        if s1[i - 1] == s2[j - 1] {
            ans[l] = s1[i - 1];
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] < dp[i][j - 1] {
            ans[l] = s1[i - 1];
            i -= 1;
        } else {
            ans[l] = s2[j - 1];
            j -= 1;
        }
    }
    while i > 0 {
        l -= 1;
        ans[l] = s1[i - 1];
        i -= 1;
    }
    while j > 0 {
        l -= 1;
        ans[l] = s2[j - 1];
        j -= 1;
    }
    ans.iter().collect::<String>()
}
