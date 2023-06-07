// https://leetcode.com/problems/shortest-path-visiting-all-nodes/solutions/490216/a-rust-solution-aspired-by/
///思路：https://zxi.mytechroad.com/blog/graph/leetcode-847-shortest-path-visiting-all-nodes/
///用当前节点和已经访问过的节点的元祖来做访问记录，防止重复访问
impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;
        let node_num = graph.len();
        let success_flag = (1<<node_num) -1;
        let mut queue = VecDeque::new();
        //初始化时将所有节点记录
        for pos in 0..node_num {
            queue.push_back((pos,1<<pos));
        }
        let mut visited = vec![vec![0;1<<node_num];node_num];
        let mut steps = 0i32;
        while !queue.is_empty(){
            //记录本次操作需要进行的数量；
            let current_num = queue.len();
            for _ in 0..current_num {
                let (current,state) = queue.pop_front().unwrap();
                if state == success_flag {
                    return steps;
                }
                if visited[current][state] == 1 {
                    continue;
                }
                visited[current][state] = 1;
                //将下一步操作的相连接节点塞入队列
                for &node in &graph[current]{
                    queue.push_back((node as usize,state | (1<<node)))
                }
            }
            steps += 1;
        }
        -1
    }
}