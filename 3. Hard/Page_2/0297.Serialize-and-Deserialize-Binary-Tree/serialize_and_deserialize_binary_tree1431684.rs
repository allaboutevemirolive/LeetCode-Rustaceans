// https://leetcode.com/problems/serialize-and-deserialize-binary-tree/solutions/1431684/c-solution-using-level-order/
public class Codec {
    //Serialization based on level order traversal   
    //Splitter: * splits the level.
    //          , splits the children belong to different parent
    //          # splits left and right child of same parent
    //          $ indicate current node is null
    
    
        // Encodes a tree to a single string.
        public string serialize(TreeNode root)
        {
            if (root == null) return "$";
            var queue = new Queue<TreeNode>();
            queue.Enqueue(root);
            var sb = new StringBuilder();
            sb.Append(root.val);
            var tempsb = new StringBuilder();
            sb.Append("*");
            var width = 1;
            while (queue.Count > 0)
            {
                var count = 0;
                var nc = 0;
                while (count < width)
                {
                    var node = queue.Dequeue();
                    if (node.left != null)
                    {
                        tempsb.Append(node.left.val);
                        queue.Enqueue(node.left);
                        nc++;
                    }
                    else
                    {
                        tempsb.Append("$");
                    }
                    tempsb.Append("#");
                    if (node.right != null)
                    {
                        tempsb.Append(node.right.val);
                        queue.Enqueue(node.right);
                        nc++;
                    }
                    else
                    {
                        tempsb.Append("$");
                    }
                    if (count < width - 1)
                    {
                        tempsb.Append(",");
                    }

                    count++;
                }
                sb.Append(tempsb.ToString());
                tempsb.Clear();
                if (queue.Count > 0)
                {
                    sb.Append("*");
                }
                width = nc;
            }
            return sb.ToString();
        }

        // Decodes your encoded data to tree.
        public TreeNode deserialize(string data)
        {
            var level = data.Split("*").ToList();
            if (level[0] == "$") return null;
            var root = new TreeNode(int.Parse(level[0]));
            var queue = new Queue<TreeNode>();
            queue.Enqueue(root);
            for (var i =1; i<level.Count; i++)
            {
                var s = level[i];
                foreach (var ch in s.Split(","))
                {
                    if (ch == "$#$")
                    {
                        queue.Dequeue();
                        continue;
                    }
                    var children = ch.Split("#").ToList();
                    queue.Peek().left = children[0] == "$" ? null : new TreeNode(int.Parse(children[0]));
                    if (queue.Peek().left != null) queue.Enqueue(queue.Peek().left);
                    queue.Peek().right = children[1] == "$" ? null : new TreeNode(int.Parse(children[1]));
                    if (queue.Peek().right != null) queue.Enqueue(queue.Peek().right);
                    queue.Dequeue();
                }
            }
            return root;
        }
}