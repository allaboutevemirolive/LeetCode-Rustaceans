// https://leetcode.com/problems/wildcard-matching/solutions/3527564/c-recursion-memoization/
class Solution {
public:
    
    int f(int i,int j,string &s,string &p,vector<vector<int>> &dp){

        if(i>=s.size() || j>=p.size()){
            if(i>=s.size() && j>=p.size()){
                return 1;
            }
            
            if(i>=s.size()){
                for(int k=j;k<p.size();++k){
                    if(p[k]!='*'){
                        return 0;
                    }
                }
                return 1;
            }

            return 0;
        }

        if(dp[i][j]!=-1){
            return dp[i][j];
        }

        int take=0;
        if(p[j]=='*'){
            for(int k=i;k<s.size();++k){
                take= take | f(k,j+1,s,p,dp) | f(k+1,j+1,s,p,dp);
            }
        }else if(p[j]=='?'){
            take=take | f(i+1,j+1,s,p,dp);
        }else{
            if(s[i]==p[j]){
                take=1 & f(i+1,j+1,s,p,dp);
            }else{
                take=0;
            }
        }
        return dp[i][j]=take;
    }

    bool isMatch(string s, string p) {
        int n=s.size(),m=p.size();
        vector<vector<int>>dp (n+1,vector<int>(m+1,-1));
        return f(0,0,s,p,dp);

    }
};