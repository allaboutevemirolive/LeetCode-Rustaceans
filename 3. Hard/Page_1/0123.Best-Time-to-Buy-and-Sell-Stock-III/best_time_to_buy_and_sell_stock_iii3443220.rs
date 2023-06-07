// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/solutions/3443220/interesting-solution/
class Solution {
public:
    int maxProfit(vector<int>& prices) 
    {
        int n = prices.size();
        if(n == 0)
        {
            return 0;
        }
        vector<int> left(n);
        vector<int> right(n);
        //from left to right
        int minim = prices[0];
        for(int i = 1; i < n; i++)
        {
            if(prices[i] < minim)
            {
                minim = prices[i];
            }
            int profit = prices[i] - minim;
            left[i] = max(profit, left[i - 1]);
        }
        int maxim = prices[n - 1];
        for(int i = n - 2; i>= 0; i--)
        {
            if(prices[i] > maxim)
            {
                maxim = prices[i];
            }
            int profit = maxim - prices[i];
            right[i] = max(profit, right[i + 1]);
        }
        int MaxProfit = 0;
        for(int i = 0; i < n; i++)
        {
            MaxProfit = max(MaxProfit, left[i] + right[i]);
        }
        return MaxProfit;
    }
};