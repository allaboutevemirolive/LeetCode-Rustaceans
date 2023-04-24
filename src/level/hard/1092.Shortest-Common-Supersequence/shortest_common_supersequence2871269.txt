// https://leetcode.com/problems/shortest-common-supersequence/solutions/2871269/rust-dp-with-recursion/
impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let (n1, n2) = (str1.len(), str2.len());
        let s1 = str1.chars().collect::<Vec<char>>();
        let s2 = str2.chars().collect::<Vec<char>>();
        
        let mut dp = vec![vec![-1; n2 + 1]; n1 + 1];
        println!["{}", Self::eval(&s1, &s2, n1, n2, &mut dp)];
        
        let mut ret = vec![' '; Self::eval(&s1, &s2, n1, n2, &mut dp) as usize];
        Self::construct(&s1, &s2, n1, n2, &mut dp, &mut ret);
        
        ret.iter().collect()
    }
    
    fn eval(s1: &Vec<char>, s2: &Vec<char>, i: usize, j: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
        if dp[i][j] != -1 { return dp[i][j] }
        
        if i == 0 || j == 0 { 
            dp[i][j] = i as i32 + j as i32;
            return dp[i][j]
        }
        
        Self::eval(s1, s2, i - 1, j - 1, dp);
        Self::eval(s1, s2, i - 1, j, dp);
        Self::eval(s1, s2, i, j - 1, dp);
        
        if s1[i - 1] == s2[j - 1] {
            dp[i][j] = 1 + dp[i - 1][j - 1];
            return dp[i][j]
        }
        
        dp[i][j] = dp[i - 1][j - 1] + 2;
        dp[i][j] = dp[i][j].min(dp[i - 1][j] + 1);
        dp[i][j] = dp[i][j].min(dp[i][j - 1] + 1);
        
        dp[i][j]
    }
    
    fn construct(s1: &Vec<char>, s2: &Vec<char>, i: usize, j: usize, dp: &mut Vec<Vec<i32>>, ret: &mut Vec<char>) {
        if i == 0 {
            for k in 0 .. j { ret[k] = s2[k]; }
            return
        }
        
        if j == 0 {
            for k in 0 .. i { ret[k] = s1[k]; }
            return
        }
        
        let k = dp[i][j] as usize - 1;
        
        if s1[i - 1] == s2[j - 1] {
            ret[k] = s1[i - 1];
            Self::construct(s1, s2, i - 1, j - 1, dp, ret);
            return
        }
        
        if dp[i][j] == dp[i - 1][j - 1] + 2 {
            ret[k] = s1[i - 1];
            ret[k - 1] = s2[j - 1];
            Self::construct(s1, s2, i - 1, j - 1, dp, ret);
            return
        }
        
        if dp[i][j] == dp[i - 1][j] + 1 {
            ret[k] = s1[i - 1];
            Self::construct(s1, s2, i - 1, j, dp, ret);
            return
        }
        
        ret[k] = s2[j - 1];
        Self::construct(s1, s2, i, j - 1, dp, ret);
    }
}