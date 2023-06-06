// https://leetcode.com/problems/trapping-rain-water/solutions/3596359/c-o-n-two-poiners-explained/
class Solution {
public:
    int trap(vector<int>& height) {
    int waterTrapped = 0;
    int left = 0, right = height.size() - 1;
    int leftMax = 0, rightMax = 0;

    while (left < right) {
        if (height[left] < height[right]) {
            // Calculate water trapped on the left side
            if (height[left] >= leftMax) {
                leftMax = height[left];
            } else {
                waterTrapped += leftMax - height[left];
            }
            left++;
        } else {
            // Calculate water trapped on the right side
            if (height[right] >= rightMax) {
                rightMax = height[right];
            } else {
                waterTrapped += rightMax - height[right];
            }
            right--;
        }
    }

    return waterTrapped;
    }
};