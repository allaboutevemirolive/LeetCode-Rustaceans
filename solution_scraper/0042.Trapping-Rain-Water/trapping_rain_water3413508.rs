// https://leetcode.com/problems/trapping-rain-water/solutions/3413508/o-n-java-code-with-proper-explanation-and-approach/
class Solution {
    public int trap(int[] arr) {
        int maxi = 0;//index with maximum height value
        int max = Integer.MIN_VALUE;
        for(int i =0;i<arr.length;i++){
            if(arr[i]>=max){
                max = arr[i];
                maxi = i;
            }
        }//maxi now stores the index with maximum height value
        int[] arr1 = new int[maxi+1];//one array with the first half of the heights
        int[] arr2 = new int[arr.length-maxi];//second array with the second half of heights - includes maxi
        for(int i=0;i<=maxi;i++)
            arr1[i] = arr[i];
        for(int i=0;i<arr2.length;i++)
            arr2[i] = arr[arr.length-i-1];//second array is filled in reverse since water filled is same regardless of order. we need to do this as the fill function only works if the array has the max value at the end
        return fill(arr1)+fill(arr2);
    }
    public static int fill(int[] arr){
        int start = 0; //indicates the starting height of fill
        int starti = 0; //indicates the starting index of fill
        int fill = 0; //indicates whether we are filling or not
        int fullwater =0; //indicates the total amount of water filled
        for(int i = 1;i<arr.length;i++){
            if(fill==0){
                if(arr[i-1]>arr[i]){//we detect filling by seeing if there is a fall in height
                    fill = 1;
                    start = arr[i-1];
                    starti = i-1;
                }
            }
            if(fill==1){
                if(arr[i]>=start){//if the height exceeds the starting height, then the current puddle has been filled
                    fill = 0;//we are not filling anymore
                }
                else{
                    fullwater += start-arr[i];//since fill status is 1 height of water filled is starting height - current height
                }//Note that this doesnt cover the cases where the height of water is limited by the ending height of puddle. the way we have constructed the arrays makes sure that will never happen
            }
        }
        return fullwater;
    }
}