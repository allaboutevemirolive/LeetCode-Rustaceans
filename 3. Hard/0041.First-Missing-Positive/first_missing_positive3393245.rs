// https://leetcode.com/problems/first-missing-positive/solutions/3393245/easy-2-line-of-code-in-c/
class Solution {
public:
    int firstMissingPositive(vector<int>& nums)
     {
        int first=1;
        int n=nums.size();
        sort(nums.begin(),nums.end());
        for(int i=0;i<n;i++)
        {
            if(nums[i]== first)
            {
                first++;
            }
            
        }
        return first;
        
    }
};