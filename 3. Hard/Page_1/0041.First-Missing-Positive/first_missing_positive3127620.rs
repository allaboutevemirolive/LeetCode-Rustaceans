// https://leetcode.com/problems/first-missing-positive/solutions/3127620/easy-peasy-soln-to-hard-problem/
class Solution {
public:
    int firstMissingPositive(vector<int>& nums) 
    {
        int n = nums.size();

        for(int i = 0; i < n; i++)
        {
            if(nums[i] < 0)
              nums[i] = 0;
        }

        int ones = 0;
        for(auto x: nums)
        {
            if(x == 1)
               ones++;  
        }

        if(ones == 0)
            return 1;

        sort(nums.begin(), nums.end());
        int ele = -1;

        for(int i = 1; i < n; i++)
        {
            if(nums[i] - nums[i-1] > 1)
            {
                ele = nums[i-1] + 1;
                break;
            }
        } 

        return ele == -1 ? ++nums[n-1] : ele;     
    }
};