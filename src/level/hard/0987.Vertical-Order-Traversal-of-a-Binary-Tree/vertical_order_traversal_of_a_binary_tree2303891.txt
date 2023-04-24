// https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree/solutions/2303891/c-rust-100-faster/
class Solution {
public:
	vector<vector<int>> verticalTraversal(TreeNode* root) {
		vector<vector<int>> ans;
		if(!root)return ans;
		int mn = INT_MAX;        
		int mx = INT_MIN;
		unordered_map<int,vector<int>> res;
		queue<pair<TreeNode*,int>> q;
		q.push({root,0});
		while(!q.empty()){
			int n = q.size();
			unordered_map<int,vector<int>> um;
			while(n--) {
				int d = q.front().second;            
				TreeNode* cur = q.front().first;
				q.pop();
				um[d].push_back(cur->val);
				if(cur->left)q.push({cur->left, d-1});
				if(cur->right)q.push({cur->right, d+1});
				if(mn>d)mn=d;
				if(mx<d)mx=d;
			}
			for(auto it:um){
				sort(it.second.begin(),it.second.end());
				for(int& x : it.second){
					res[it.first].push_back(x);
				}
			}
		}
		for(int i=mn; i<=mx; i++){
			ans.push_back(res[i]);
		}
		return ans;
	}
};