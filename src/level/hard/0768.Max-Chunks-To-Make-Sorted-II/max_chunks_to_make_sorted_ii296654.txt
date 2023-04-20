// https://leetcode.com/problems/max-chunks-to-make-sorted-ii/solutions/296654/simpler-linear-java-solution/
class Solution {
    public int maxChunksToSorted(int[] arr) {
        int max[] = new int[arr.length + 1];
        max[0] = Integer.MIN_VALUE;
        for (int i = 0; i < arr.length; i++) {
            max[i + 1] = Math.max(max[i], arr[i]);
        }
        
        int count = 0;
        int min = Integer.MAX_VALUE;
        for (int i = arr.length - 1; i >=0; i--) {
            min = Math.min(min, arr[i]);
            if (max[i] <= arr[i] && max[i]<=min) {
                count++;                
            }
        }
        
        return count;
    }
}