// https://leetcode.com/problems/trapping-rain-water/solutions/1374608/c-java-python-maxleft-maxright-so-far-with-picture-o-1-space-clean-concise/
class Solution:
    def trap(self, height: List[int]) -> int:
        n = len(height)
        l_max = [0] * n
        r_max = [0] * n
        for i in range(1, n):
            l_max[i] = max(l_max[i], l_max[i-1])
        for i in range(n - 2, -1, -1):
            r_max[i] = max(r_max[i], r_max[i+1])  
        return sum(max(min(l_max[i], r_max[i]) - height[i], 0) for i in range(n))