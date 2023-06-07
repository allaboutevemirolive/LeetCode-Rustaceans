// https://leetcode.com/problems/consecutive-numbers-sum/solutions/1155042/recursion-and-dp-with-tle-because-what-if-i-can-t-get-mathematical/
class Solution {
    int ways = 0;
    public int consecutiveNumbersSum(int N) {
        
        for (int i = 1; i <= N; i++) {
            backtrack(i, 0, N);
        }
        
        return ways;
    }
    
    public void backtrack(int number, long sum, int N) {
        if (sum > N)
            return;
        if (sum == N) {
            ways++;
            return;
        }
        
        backtrack(number + 1, sum + number, N);
    }
}