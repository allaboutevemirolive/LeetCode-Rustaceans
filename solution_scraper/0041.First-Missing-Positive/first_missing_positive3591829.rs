// https://leetcode.com/problems/first-missing-positive/solutions/3591829/very-very-easy-java-soln-0ms/
class Solution {
    public int firstMissingPositive(int[] nums) {
        int n=nums.length;
        Arrays.sort(nums);
        int b=1;
        for(int i=0;i<n;i++)
        {
            if(nums[i]==b)
            {
               b++;
            }
            
        }
        return b;
        
    }
}