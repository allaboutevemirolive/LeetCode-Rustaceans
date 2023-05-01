// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/solutions/2280587/c-rust-explained/
class Solution {
public:
	TreeNode* buildTree(vector<int>& inorder, vector<int>& postorder) {
		int n = inorder.size();
		unordered_map<int,int> um;
		for(int i=0;i<n;i++) {
			um[inorder[i]]=i; 
		}
		return f(inorder,0,n-1,postorder,0,n-1,um);
	}
	TreeNode* f(vector<int>& inorder, int is, int ie, vector<int>& postorder, int ps, int pe, unordered_map<int,int>& um) {
		if(is > ie || ps > pe)return NULL;
		TreeNode* root = new TreeNode(postorder[pe]);
		int ind = um[root->val];
		int len = ind - is;
		root->left = f(inorder,is,ind-1,postorder,ps,ps+len-1,um);        
		root->right = f(inorder,ind+1,ie,postorder,ps+len,pe-1,um);
		return root;
	}
};