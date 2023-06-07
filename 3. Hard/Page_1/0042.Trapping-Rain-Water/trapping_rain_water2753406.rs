// https://leetcode.com/problems/trapping-rain-water/solutions/2753406/c-simple-way-only-one-array/
public class Solution {
    public int Trap(int[] height) {
            int st = 0;
            int tmpRain = 0;
            int totalRain = 0;
            bool close = false;

            for (int i = 0; i < height.Length; i++) {

                if (height[i] > 0) {
                    st = height[i];

                    do {
                        tmpRain = 0;
                        close = false;

                        for (int j = i + 1; j < height.Length; j++) {
                            if (st > height[j]) {
                                tmpRain += st - height[j];
                            }
                            else {
                                totalRain += tmpRain;
                                i = j - 1;
                                close = true;
                                break;
                            }
                        }

                        if(close == false) {
                            st--;

                            if (height.Length > i+1) {
                                if (st == height[i + 1]) {
                                    break;
                                }
                            }
                        }

                    } while (st > 0 && close ==false);
                }
            }

            return totalRain;
    }
}