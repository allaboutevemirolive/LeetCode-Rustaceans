// https://leetcode.com/problems/trapping-rain-water/solutions/3387829/c-java-python-javascript-o-n-time-o-1-space-brute-force-optimized-code/
class Solution {
public:
    int trap(vector<int>& height) {
        int n = height.size();
         int water=0;
         for(int i=0;i<height.size();i++){
             int left_max=0,right_max=0;
             int j=i;
             while(j<n){
                 right_max=max(right_max,height[j]);
                 j++;
             }
             j=i;
             while(j>=0){
                 left_max=max(left_max,height[j]);
                 j--;
             }
             j=i;
             water+= min(left_max,right_max)-height[i];
        }
        return water;
    }
};