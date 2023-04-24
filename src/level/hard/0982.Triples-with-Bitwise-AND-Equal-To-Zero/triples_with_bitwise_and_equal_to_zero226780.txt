// https://leetcode.com/problems/triples-with-bitwise-and-equal-to-zero/solutions/226780/c-132ms-o-n-2-14-lines/
class Solution {
public:
    int countTriplets(vector<int>& A) {
        vector<int> v((1<<16),0);
        for(int i=0;i<A.size();i++)
            for(int j=0;j<A.size();j++)
                v[A[i] & A[j]]++;
        int ans = 0;
        for(int i=0;i<A.size();i++)
            for(int j=0;j<v.size();j++)
                if((A[i] & j) == 0) ans += v[j];
        return ans;
    }
};