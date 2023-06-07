// https://leetcode.com/problems/minimum-insertion-steps-to-make-a-string-palindrome/solutions/3442568/rust-explanation-quadratic-speed-linear-memory/
if s[i] == s[j]:
  DP[i][j] = DP[i + 1][j - 1]
else:
  DP[i][j] = 1 + min(DP[i + 1][j], DP[i][j - 1]