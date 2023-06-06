// https://leetcode.com/problems/wildcard-matching/solutions/3416314/c-dp-recursive-memoization/
class Solution {
public:
    bool c(string &s,string &p,int i,int j,vector<vector<int>>&dp){
        if(i==s.size()){
            int t=0;
            for(int g=j;g<p.size();g++)if(p[g]=='*')t++;
            if(t==(p.size()-j))return true;
            else return false;
        }
        
        if(j>=p.size() || i>s.size())return false;
        if(dp[i][j]!=-1)return dp[i][j];
        bool ans=false;
        if(p[j]>='a' && p[j]<='z'){
           if(s[i]!=p[j])ans=false;
           else ans|= c(s,p,i+1,j+1,dp);
        }
        else if(p[j]=='?')ans|= c(s,p,i+1,j+1,dp);
        else {
            for(int h=i;h<s.size();h++){
                    ans |= c(s,p,h+1,j+1,dp);
                    ans |= c(s,p,h,j+1,dp);
            }
        }
        dp[i][j]=ans;
        return ans;
    
    }
    bool isMatch(string s, string p) {
        vector<vector<int>>dp(s.size(),vector<int>(p.size(),-1));
        return c(s,p,0,0,dp);
    }
};