// https://leetcode.com/problems/design-skiplist/solutions/2356192/rust-use-vec-rc-refcell/
use std::{cell::RefCell, rc::Rc, vec};

use rand::Rng;

const MAX_LEVEL: usize = 32;
const P_FACTOR: f64 = 0.25;

type Link = Option<Rc<RefCell<Node>>>;

fn new_link(value: i32, level: usize) -> Link {
    Some(Rc::new(RefCell::new(Node::new(value, level))))
}

fn random_level() -> usize {
    let mut level = 1;
    let mut rng = rand::thread_rng();
    while level < MAX_LEVEL && rng.gen::<f64>() < P_FACTOR {
        level += 1;
    }
    level
}

#[derive(Debug, Default)]
pub struct Node {
    val: i32,
    level: usize,
    forward: Vec<Link>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val && self.forward.len() == other.forward.len()
    }
}

impl Node {
    pub fn new(value: i32, level: usize) -> Node {
        Node {
            val: value,
            level: level,
            forward: vec![None; level],
        }
    }
}

#[derive(Debug, Default)]
pub struct Skiplist {
    head: Link,
    level: usize,
}

impl Skiplist {
    pub fn new() -> Self {
        Skiplist {
            head: new_link(-1, MAX_LEVEL),
            ..Default::default()
        }
    }

    pub fn search(&self, target: i32) -> bool {
        let mut cur = self.head.clone().unwrap();
        for i in (0..self.level).rev() {
            loop {
                match cur.clone().borrow().forward[i].clone() {
                    Some(node) => {
                        if node.borrow().val < target {
                            cur = node;
                            continue;
                        }
                        break;
                    }
                    None => break,
                }
            }
        }
        match cur.clone().borrow().forward[0].clone() {
            Some(node) => {
                if node.borrow().val == target {
                    return true;
                }
                false
            }
            None => false,
        }
    }

    pub fn add(&mut self, num: i32) {
        let mut cur = self.head.clone().unwrap();
        let mut update = vec![Some(cur.clone()); MAX_LEVEL];
        for i in (0..self.level).rev() {
            loop {
                match cur.clone().borrow().forward[i].clone() {
                    Some(node) => {
                        if node.borrow().val < num {
                            cur = node;
                            continue;
                        }
                        break;
                    }
                    None => break,
                }
            }
            update[i] = Some(cur.clone());
        }
        let level = random_level();
        self.level = self.level.max(level);
        let node = new_link(num, level);
        for i in 0..level {
            node.as_ref().unwrap().borrow_mut().forward[i] =
                update[i].as_ref().unwrap().borrow().forward[i].clone();
            update[i].as_ref().unwrap().borrow_mut().forward[i] = node.clone();
        }
    }

    pub fn erase(&mut self, num: i32) -> bool {
        let mut cur = self.head.clone().unwrap();
        let mut update: Vec<Link> = vec![None; MAX_LEVEL];
        for i in (0..self.level).rev() {
            loop {
                match cur.clone().borrow().forward[i].clone() {
                    Some(node) => {
                        if node.borrow().val < num {
                            cur = node;
                            continue;
                        }
                        break;
                    }
                    None => break,
                }
            }
            update[i] = Some(cur.clone());
        }
        let cur = cur.clone().borrow().forward[0].clone();
        match cur.clone() {
            Some(node) => {
                if node.borrow().val != num {
                    return false;
                }
            }
            None => return false,
        };

        for i in 0..self.level {
            if update[i].clone().unwrap().borrow().forward[i] != cur.clone() {
                break;
            }
            update[i].clone().unwrap().borrow_mut().forward[i] =
                cur.clone().unwrap().borrow().forward[i].clone();
        }
        while self.level > 1
            && self.head.clone().unwrap().borrow().forward[self.level - 1].is_none()
        {
            self.level -= 1;
        }

        return true;
    }

    pub fn debug(&self) {
        let head = self.head.clone().unwrap();
        let mut table: Vec<Vec<(i32, usize, usize)>> = vec![
            vec![(
                head.borrow().val,
                head.borrow().level,
                head.borrow().forward.len()
            )];
            self.level
        ];

        for i in (0..self.level).rev() {
            let mut node = head.clone();
            loop {
                match node.clone().borrow().forward[i].clone() {
                    Some(tmp) => {
                        table[i].push((
                            tmp.borrow().val,
                            tmp.borrow().level,
                            tmp.borrow().forward.len(),
                        ));
                        node = tmp;
                    }
                    None => break,
                }
            }
        }
        for t in table.iter().rev() {
            println!("{:?}", t);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const N: i32 = 10;
    #[test]
    fn test_skiplist() {
        let mut rng = rand::thread_rng();
        let mut skiplist = Skiplist::new();
        let mut v = vec![];
        for _ in 0..N {
            let num = rng.gen_range(0..N);
            v.push(num);
            skiplist.add(num);
        }
        skiplist.debug();

        for _ in 0..N {
            let num = rng.gen_range(0..N);
            assert_eq!(v.contains(&num), skiplist.search(num));
        }
        for _ in 0..N {
            let num = rng.gen_range(0..N);
            println!("=====");
            println!("{}", num);
            match v.iter().position(|&x| x == num) {
                Some(index) => {
                    assert!(skiplist.erase(num));
                    v.remove(index);
                }
                None => assert!(!skiplist.erase(num)),
            }
            skiplist.debug()
        }
    }
}