// https://leetcode.com/problems/lfu-cache/solutions/697319/rust-o-1-100-time-memory/
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct ListNode {
    key: i32,
    prev: Option<Rc<RefCell<ListNode>>>,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    fn new(key: i32) -> Self {
        Self {
            key,
            prev: None,
            next: None,
        }
    }

    fn pluck(&mut self) {
        if let Some(node) = &self.prev {
            node.borrow_mut().next = self.next.as_ref().map(|r| r.clone());
        }

        if let Some(node) = &self.next {
            node.borrow_mut().prev = self.prev.as_ref().map(|r| r.clone());
        }

        self.next = None;
        self.prev = None;
    }
}

struct FrequencyBin {
    val: i32,
    count: i32,
    map: HashMap<i32, Rc<RefCell<ListNode>>>,
    head: Option<Rc<RefCell<ListNode>>>,
    tail: Option<Rc<RefCell<ListNode>>>,
}

impl FrequencyBin {
    fn new(val: i32) -> Self {
        Self {
            val,
            count: 0,
            map: HashMap::new(),
            head: None,
            tail: None,
        }
    }

    fn remove_key(&mut self, key: i32) {
        if let Some(node) = self.map.get_mut(&key) {
            if self.tail.as_ref().map_or(false, |n| Rc::ptr_eq(&n, node)) {
                self.tail = node.borrow().prev.as_ref().map(|r| r.clone());
            }

            if self.head.as_ref().map_or(false, |n| Rc::ptr_eq(&n, node)) {
                self.head = node.borrow().next.as_ref().map(|r| r.clone());
            }

            node.borrow_mut().pluck();
            self.map.remove(&key);
            self.count -= 1;
        }
    }

    fn push_back(&mut self, key: i32) {
        let new_node = Rc::new(RefCell::new(ListNode::new(key)));
        if let Some(node) = &self.tail {
            node.clone().borrow_mut().next = Some(new_node.clone());
            new_node.borrow_mut().prev = Some(node.clone());
            self.tail = Some(new_node.clone())
        } else {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node.clone());
        }
        self.map.insert(key, new_node.clone());
        self.count += 1;
    }

    fn pop_front(&mut self) {
        let key = self.head.as_ref().unwrap().borrow().key;
        self.remove_key(key);
    }
}

struct CacheItem {
    value: i32,
    freq: i32
}

struct LFUCache {
    capacity: i32,
    count: i32,
    lowest_freq: i32,
    item_map: HashMap<i32, CacheItem>,
    freq_map: HashMap<i32, FrequencyBin>,
}

impl LFUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity,
            count: 0,
            lowest_freq: 0,
            item_map: HashMap::new(),
            freq_map: HashMap::new(),
        }
    }

    fn evict(&mut self) {
        let lowest_freq = self.freq_map.get_mut(&self.lowest_freq).unwrap();
        self.item_map
            .remove(&lowest_freq.head.as_ref().unwrap().borrow().key);
        lowest_freq.pop_front();
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(item) = self.item_map.get_mut(&key) {
            let old_freq = self.freq_map.get_mut(&item.freq).unwrap();
            old_freq.remove_key(key);
            if self.lowest_freq == old_freq.val && old_freq.count == 0 {
                self.lowest_freq += 1;
            }
    
            item.freq += 1;
    
            self.freq_map
                .entry(item.freq)
                .or_insert(FrequencyBin::new(item.freq))
                .push_back(key);
            item.value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(item) = self.item_map.get_mut(&key) {
            let old_freq = self.freq_map.get_mut(&item.freq).unwrap();
            old_freq.remove_key(key);
            if self.lowest_freq == old_freq.val && old_freq.count == 0 {
                self.lowest_freq += 1;
            }
            
            item.freq += 1;
            item.value = value;

            self.freq_map
                .entry(item.freq)
                .or_insert(FrequencyBin::new(item.freq))
                .push_back(key);
        } else {
            if self.capacity > 0 {
                if self.count == self.capacity {
                    self.evict();
                    self.count -= 1;
                }

                self.item_map.insert(key, CacheItem { value, freq: 1 });
                self.freq_map
                    .entry(1)
                    .or_insert(FrequencyBin::new(1))
                    .push_back(key);

                self.count += 1;
                self.lowest_freq = 1;
            }
        }
    }
}