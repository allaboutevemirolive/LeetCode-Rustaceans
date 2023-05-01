// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/solutions/2280623/c-rust-explained/
class Solution {
public:
	unordered_map<int,int> um;
	TreeNode* f(int preSt, int preEn, int inSt, int inEn, vector<int>& preorder, vector<int>& inorder){
		if(preSt > preEn || inSt > inEn) return NULL;
		TreeNode* root = new TreeNode(preorder[preSt]);
		int ind = um[root->val];
		int len = ind - inSt;
		root->left = f(preSt+1,preSt+len,inSt,ind-1,preorder,inorder);        
		root->right = f(preSt+len+1,preEn,ind+1,inEn,preorder,inorder);
		return root;
	}
	TreeNode* buildTree(vector<int>& preorder, vector<int>& inorder) {
		int n = inorder.size();
		for(int i=0;i<n;i++){
			um[inorder[i]]=i;
		}
		return f(0,n-1,0,n-1,preorder,inorder);
	}
};