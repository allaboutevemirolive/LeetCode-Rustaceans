// https://leetcode.com/problems/concatenated-words/solutions/376820/dp-c-17-unordered-map-with-string-view-340ms/
class Solution {
  
    bool canBuild(string const& s, const unordered_set<string_view>& prev) {
        if (prev.size() == 0) return false;
		
		// dp[i] answers the question "Can I build substring s[0..i] which is concatenation of other string which have processed earlier?"
        vector<bool> dp(s.size()+1, false);
        dp[0] = true;
        for (size_t i = 1; i <= s.size(); ++i) {
            for (int j = 0; j < i; ++j)
                if (dp[j] &&
                    prev.count(string_view(s.c_str()+j, i - j))) {
                    dp[i] = true;
                    break;
                }
        }
        return dp[s.size()];
    }
    
public:
    vector<string> findAllConcatenatedWordsInADict(vector<string>& words) {
        // 1. sort strings by length, O(nlogn)
        sort(words.begin(), words.end(), [](const string& l, const string& r) -> bool { return l.size() < r.size();});
        // 2. iterate over strings and check if you can build it with previously considered strings (smaller length)
        vector<string> res;
        unordered_set<string_view> previous;
        previous.reserve(words.size());
        for (string& s : words) {
            if (canBuild(s, previous))
                res.emplace_back(s);
            previous.insert(string_view(s.c_str(), s.size()));
        }
        return res;
    }
};