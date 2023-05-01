// https://leetcode.com/problems/kth-smallest-element-in-a-bst/solutions/2327747/c-rust-easy-recursion/
class Solution {
public:
	int ans = -1;
	int kthSmallest(TreeNode* root, int k) {
		int i = 0;
		inorder(root, i, k);
		return ans;
	}
	void inorder(TreeNode* root, int &i, int k){
		if(!root)return;
		inorder(root->left, i, k);
		i++;
		if(i==k) ans = root->val;
		inorder(root->right, i,k);
	}
};