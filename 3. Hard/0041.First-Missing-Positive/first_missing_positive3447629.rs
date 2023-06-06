// https://leetcode.com/problems/first-missing-positive/solutions/3447629/c-sorting-easy-10-lines-code/
class Solution {
public:
    int firstMissingPositive(vector<int>& nums) {
        int n = nums.size();
        sort(nums.begin(),nums.end()); // to sort the array 

        int curPos=1; // sets current Positive to 1 
        
        for(int i=0;i<n;i++){
            if(nums[i]==curPos) curPos++;
        }

        return curPos;

    }
};