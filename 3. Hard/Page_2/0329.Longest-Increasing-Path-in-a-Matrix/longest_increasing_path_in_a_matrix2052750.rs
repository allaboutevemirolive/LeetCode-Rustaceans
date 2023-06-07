// https://leetcode.com/problems/longest-increasing-path-in-a-matrix/solutions/2052750/easy-and-understandable-solution/
class Solution {
public:
    int len[205][205];
    int n,m;
    vector<vector<int>> v;
    void dfs(int i, int j){
        if(i < n - 1){
            if(v[i+1][j] > v[i][j]){
                if(len[i+1][j]==1){
                    dfs(i+1,j);
                }
                len[i][j] = max(len[i+1][j]+1, len[i][j]);
            }
        }
        if(j < m - 1){
            if(v[i][j+1] > v[i][j]){
                if(len[i][j+1]==1){
                    dfs(i,j+1);
                }
                len[i][j] = max(len[i][j+1]+1, len[i][j]);
            }
        }
        if(i > 0){
            if(v[i-1][j] > v[i][j]){
                if(len[i-1][j]==1){
                    dfs(i-1,j);
                }
                len[i][j] = max(len[i-1][j]+1, len[i][j]);
            }
        }
        if(j > 0){
            if(v[i][j-1] > v[i][j]){
                if(len[i][j-1]==1){
                    dfs(i,j-1);
                }
                len[i][j] = max(len[i][j-1]+1, len[i][j]);
            }
        }
    }
    int longestIncreasingPath(vector<vector<int>>& matrix) {
        v = matrix;
        n = v.size();
        m = v[0].size();
        for(int i = 0; i < 205; i++){
            for(int j = 0; j < 205; j++){
                len[i][j] = 1;
            }
        }
        int ans = 0;
        for(int i = 0; i < v.size(); i++){
            for(int j = 0; j < v[i].size(); j++){
                if(len[i][j] == 1)
                    dfs(i,j);
                ans=max(ans,len[i][j]);
            }
        }
        return ans;
    }
};