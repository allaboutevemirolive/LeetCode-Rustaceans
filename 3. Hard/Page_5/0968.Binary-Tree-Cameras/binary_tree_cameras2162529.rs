// https://leetcode.com/problems/binary-tree-cameras/solutions/2162529/unofficial-rust-0ms-c-100ms/
// C# implementation
public class Solution {
    public int MinCameraCover(TreeNode root) {
        // Results for different distance to the nearest camera
        // 1 - node has camera, can see parent
        // 2 - node don't has camera, can't see parent, but on camera itself
        // 3 - node don't has camera, not on camera and should be seen by parent camera
        static (int, int, int) traversal(TreeNode node) {
            if (node.left == null) {
                if (node.right == null)
                    return (1, 1, 0);

                var rightResult = traversal(node.right);
                return (1 + rightResult.Item3, rightResult.Item1, rightResult.Item2);
            }
            else {
                var leftResult = traversal(node.left);
                if (node.right == null)                        
                    return (1 + leftResult.Item3, leftResult.Item1, leftResult.Item2);

                var rightResult = traversal(node.right);
                var item1 = 1 + leftResult.Item3 + rightResult.Item3;
                var item2 = Math.Min(item1, Math.Min(leftResult.Item1 + rightResult.Item2, leftResult.Item2 + rightResult.Item1));
                var item3 = Math.Min(item2, leftResult.Item2 + rightResult.Item2);
                return (item1, item2, item3);
            }
        }        
        return traversal(root).Item2;
    }
}