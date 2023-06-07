// https://leetcode.com/problems/remove-boxes/solutions/1183986/c-and-rust-dp/
class Solution {
public:
    int removeBoxes(vector<int>& boxes) {
        int qsize = 0;
        q[0] = {boxes[0], 0};
        for (auto &b : boxes) {
            if (q[qsize].first != b)
                q[++qsize] = {b, 0};
            q[qsize].second++;
        }
              
        function<int(int,int,int)> dc = [&] (int l, int r, int k) {
            auto &ans = dp[l][r][k];
            
            if (ans || l >= r)
                return ans;
            
            ans = (k + q[l].second) * (k + q[l].second) + dc(l + 1, r, 0);
            for (int i = l + 1; i < r; i++) {
                if (q[i].first != q[l].first)
                    continue;
                int tmp = dc(l + 1, i, 0) + dc(i, r, q[l].second + k);
                ans = max(ans, tmp);
            }
            return ans;
        };
        return dc(0, qsize + 1, 0);
    }
private:
    pair<int, int> q[101];
    int dp[101][101][101];
};