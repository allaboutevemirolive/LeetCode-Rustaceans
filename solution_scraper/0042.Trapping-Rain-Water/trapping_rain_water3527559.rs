// https://leetcode.com/problems/trapping-rain-water/solutions/3527559/most-efficient-way-t-c-o-n-s-c-o-1-to-solve-this-problem-in-java-with-comments/
class Solution {
    public int trap(int[] height) {
        //Approach: Using TwoPointera, T.C: O(n), S.C: O(1)

        //base case
        if(height==null) return 0;

        //initialize the variables
        int l = 0, r = height.length-1;
        int leftMax = height[l], rightMax = height[r];
        int ans = 0;

        //traverse, 
        while(l<r){
            if(leftMax<rightMax){
                //increment the left pointer
                l+=1;
                //find the left max
                leftMax = Math.max(leftMax, height[l]);
                //use the formula to count the the amount of water store
                ans+=leftMax-height[l];
            }
            else{
                r-=1;
                rightMax = Math.max(rightMax, height[r]);
                ans+=rightMax-height[r];
            }
        }
        return ans;
    }
}