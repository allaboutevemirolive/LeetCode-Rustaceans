// https://leetcode.com/problems/flatten-binary-tree-to-linked-list/solutions/2318977/c-rust-100-morris-traversal-o-n-o-1/
class Solution {
public:
	void flatten(TreeNode* root) {
		TreeNode* cur = root;
		TreeNode* pre = NULL;
		while(cur){
			if(cur->left){
				pre = cur->left;
				while(pre->right)pre = pre->right;
				pre->right = cur->right;
				cur->right = cur->left;
				cur->left = NULL;
			}
			cur = cur->right;
		}
	}
};