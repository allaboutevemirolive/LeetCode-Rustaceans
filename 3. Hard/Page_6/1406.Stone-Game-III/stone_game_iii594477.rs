// https://leetcode.com/problems/stone-game-iii/solutions/594477/java-dp-o-n-trying-my-best-to-explain/
class Solution {
    public String stoneGameIII(int[] stoneValue) {
        int n = stoneValue.length;
        int[][] dp = new int[n+3][2];
        for(int i = n-1; i >= 0; i--) {
            int sum = 0;
            int bestSum = Integer.MIN_VALUE;
            int nOfStones = 0;
            for(int j = i; j < i+3; j++) {
                sum += j<n?stoneValue[j]:0;
                if (sum+dp[j+1+dp[j+1][1]][0] > bestSum) {
                    bestSum = sum+dp[j+1+dp[j+1][1]][0];
                    nOfStones=j-i+1;
                    dp[i][0]=bestSum;
                    dp[i][1]=nOfStones;
                }
            }
        }
        return dp[0][0] > dp[dp[0][1]][0] ? "Alice" : (dp[0][0] == dp[dp[0][1]][0] ? "Tie" : "Bob");
    }
}