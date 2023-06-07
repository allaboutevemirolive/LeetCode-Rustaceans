// https://leetcode.com/problems/restore-the-array/solutions/3465976/rust-dp/
class Solution {
public:
    long long numberOfArrays(string s, long long k) {
        const long long modulo = 1e9 + 7;
        const long long p = static_cast<long long>(floor(log10(k + .0)) + 1);
        vector<vector<long long>> can(s.length(), vector<long long>(p, 0));

        for (long long i = 0; i < s.length(); i++) {
            long long cur = 0;
            for (long long j = 1; j <= p; j++) {
                if (i + j - 1 < s.length()) {
                    auto t = s[i + j - 1] - '0';
                    if (j == 1 && t == 0) {
                        break;
                    }
                    cur = cur * 10 + t;
                    if (cur <= k) {
                        can[i][j - 1] = 1;
                    }
                }
            }
        }

        vector<long long> dp(s.length(), 0);
        for (long long i = 0; i < s.length(); i++) {
            for (long long j = 1; j <= p; j++) {
                if (i - j >= 0) {
                    dp[i] =
                        (dp[i] + dp[i - j] * can[i - j + 1][j - 1]) % modulo;
                } else {
                    dp[i] = (dp[i] + can[max((long long)0, i - j + 1)][j - 1]) % modulo;
                    break;
                }
            }
        }

        return dp[s.length() - 1];
    }
};