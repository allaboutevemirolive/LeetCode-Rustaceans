// https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule/solutions/1301305/c-dp-clear-explanation/
class Solution {
public:
    int minDifficulty(vector<int>& a, int d) {
        int n=a.size();
        if(n<d) return -1;
        int dp[n+1][d+1];
        fill_n(&dp[0][0],(n+1)*(d+1),1e9);
        dp[0][0]=0;
        for(int i=1;i<=n;i++){
            for(int j=1;j<=d;j++){
                int mx=0;
                for(int k=i;k>=j;k--){
                    mx=max(mx,a[k-1]);
                    if(dp[k-1][j-1]<1e9)
                        dp[i][j]=min(dp[i][j],mx+dp[k-1][j-1]);
                }
            }
        }
        return dp[n][d];
    }
};