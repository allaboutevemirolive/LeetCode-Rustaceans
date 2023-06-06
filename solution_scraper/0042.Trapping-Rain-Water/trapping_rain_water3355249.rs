// https://leetcode.com/problems/trapping-rain-water/solutions/3355249/trap-rain-water/
class Solution {
    public int trap(int[] arr) {
        int ans=0;

        for(int i=0;i<arr.length;i++){
            int left =0;
            for(int j=0;j<=i;j++){
                left =Math.max(left,arr[j]);
            }
            int right = Integer.MIN_VALUE;
            for(int j=i;j<=arr.length-1;j++){
                right = Math.max(right,arr[j]);
            }
            ans += Math.min(left,right)-arr[i];
        }

        return ans;
    }
}