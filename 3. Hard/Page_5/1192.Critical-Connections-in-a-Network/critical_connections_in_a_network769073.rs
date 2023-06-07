// https://leetcode.com/problems/critical-connections-in-a-network/solutions/769073/a-slower-but-easier-to-come-up-with-solution-using-tree/
use std::collections::*;
impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        #[derive(Clone, Copy)]
        struct OutEdge {
            to: usize,
            con_id: usize,
        }
        let mut map = vec![vec![]; n];
        let mut is_branch = vec![false; connections.len()];
        
        // Build neighbor list
        
        for (con_id, con) in connections.iter().enumerate() {
            let a = con[0] as usize;
            let b = con[1] as usize;
            map[a].push(OutEdge{to: b, con_id});
            map[b].push(OutEdge{to: a, con_id});
        }
        
        #[derive(Clone)]
        struct Node {
            parent: Option<usize>,
            children: Vec<OutEdge>,
        }
        
        #[derive(Clone)]
        enum NodeEx {
            Node(Node),
            Redirect(usize),
        }
        
        impl NodeEx {
            fn node(&mut self) -> &mut Node {
                if let NodeEx::Node(node) = self {
                    node
                } else {
                    panic!()
                }
            }
        }
        
        // Build tree
        
        let mut tree = vec![NodeEx::Redirect(0); n];
        let mut queue = vec![0];
        tree[0] = NodeEx::Node(Node {
            parent: None,
            children: vec![],
        });
        while !queue.is_empty() {
            let mut new_queue = vec![];
            for cur in queue {
                for out_edge in &map[cur] {
                    if let NodeEx::Redirect(_) = &tree[out_edge.to] {
                        is_branch[out_edge.con_id] = true;
                        tree[out_edge.to] = NodeEx::Node(Node {
                            parent: Some(cur),
                            children: vec![],
                        });
                        tree[cur].node().children.push(*out_edge);
                        new_queue.push(out_edge.to);
                    }
                }
            }
            queue = new_queue;
        }
        
        // Use the rest of connection to merge tree nodes
        
        for (con, is_branch) in connections.iter().zip(is_branch) {
            if is_branch {
                continue;
            }
            let mut a = con[0] as usize;
            let mut b = con[1] as usize;
            while let NodeEx::Redirect(r) = &tree[a] {
                a = *r;
            }
            while let NodeEx::Redirect(r) = &tree[b] {
                b = *r;
            }
            
            let mut a_ancestors = HashSet::new();
            let mut a_a = a;
            loop {
                a_ancestors.insert(a_a);
                if let Some(parent) = tree[a_a].node().parent {
                    a_a = parent;
                } else {
                    break;
                }
            }
            let mut common = b;
            while !a_ancestors.contains(&common) {
                common = tree[common].node().parent.unwrap();
            }
            let mut group_node = Node {
                parent: tree[common].node().parent,
                children: vec![],
            };
            let mut a_arm = None;
            let mut b_arm = None;
            
            let mut collect_arm = |mut end: usize, arm: &mut Option<usize>| {
                while end != common {
                    for child in std::mem::replace(&mut tree[end].node().children, vec![]) {
                        if Some(child.to) == *arm {
                            continue;
                        }
                        tree[child.to].node().parent = Some(common);
                        group_node.children.push(child);
                    }
                    
                    *arm = Some(end);
                    let new_end = tree[end].node().parent.unwrap();
                    tree[end] = NodeEx::Redirect(common);
                    end = new_end;
                }
                
            };
            collect_arm(a, &mut a_arm);
            collect_arm(b, &mut b_arm);
            
            for child in &tree[common].node().children {
                if Some(child.to) == a_arm || Some(child.to) == b_arm {
                    continue;
                }
                group_node.children.push(*child);
            }
            
            tree[common] = NodeEx::Node(group_node);
        }
        
        // Collect edges
        
        let mut queue = vec![0];
        let mut result = vec![];
        while !queue.is_empty() {
            let mut new_queue = vec![];
            for node in queue {
                for child in &tree[node].node().children {
                    result.push(connections[child.con_id].clone());
                    new_queue.push(child.to);
                }
            }
            queue = new_queue;
        }
        result
    }
}