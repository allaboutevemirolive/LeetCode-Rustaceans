// https://leetcode.com/problems/poor-pigs/solutions/94300/one-line-solution-with-detailed-explanation/
public class Solution {
    public int poorPigs(int buckets, int minutesToDie, int minutesToTest) {
        return (int)Math.ceil(Math.log(buckets)/Math.log(minutesToTest/minutesToDie+1));
    }
}