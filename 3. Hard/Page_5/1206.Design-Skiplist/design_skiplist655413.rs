// https://leetcode.com/problems/design-skiplist/solutions/655413/rust-solution-with-tests-runs-in-24ms/
use core::cell::RefCell;
use std::rc::Rc;
use EntryRef::*;

pub struct Skiplist<T = i32>
where
    T: Ord,
    T: Copy,
{
    head: EntryRef<T>,
    tail: EntryRef<T>,
    height: usize,
}

#[derive(Debug)]
struct Entry<T>
where
    T: Ord,
{
    key: Option<T>,
    up: EntryRef<T>,
    down: EntryRef<T>,
    left: EntryRef<T>,
    right: EntryRef<T>,
}

impl<T> Entry<T>
where
    T: Ord,
{
    fn new(key: Option<T>) -> Self {
        Entry {
            key,
            up: Empty,
            down: Empty,
            left: Empty,
            right: Empty,
        }
    }
}

#[derive(Debug)]
enum EntryRef<T>
where
    T: Ord,
{
    Any(Rc<RefCell<Entry<T>>>),
    Empty,
}

impl<T: std::cmp::Ord> EntryRef<T> {
    fn is_none(&self) -> bool {
        match self {
            Empty => true,
            _ => false,
        }
    }

    fn is_some(&self) -> bool {
        match self {
            Empty => false,
            _ => true,
        }
    }

    fn as_ref(&self) -> Rc<RefCell<Entry<T>>> {
        match self {
            Empty => panic!("Entry is none!"),
            Any(reference) => Rc::clone(&reference),
        }
    }
}

impl<T: std::cmp::Ord> PartialEq for EntryRef<T> {
    fn eq(&self, other: &Self) -> bool {
        self.is_none() && other.is_none() || Rc::ptr_eq(&self.as_ref(), &other.as_ref())
    }
}

impl<T: std::cmp::Ord> Clone for EntryRef<T> {
    fn clone(&self) -> Self {
        match self {
            Empty => Empty,
            Any(reference) => Any(Rc::clone(reference)),
        }
    }
}

impl<T: std::cmp::Ord> PartialEq for Entry<T> {
    fn eq(&self, other: &Self) -> bool {
        other.key == self.key
            && other.left == self.left
            && other.right == self.right
            && other.up == self.up
            && other.down == self.down
    }
}

impl<T> Skiplist<T>
where
    T: Ord,
    T: Copy,
{
    pub fn new() -> Self {
        let head = Rc::new(RefCell::new(Entry::new(None)));
        let tail = Rc::new(RefCell::new(Entry::new(None)));

        head.borrow_mut().right = Any(Rc::clone(&tail));
        tail.borrow_mut().left = Any(Rc::clone(&head));

        Skiplist {
            head: Any(head),
            tail: Any(tail),
            height: 1,
        }
    }

    pub fn search(&self, target: T) -> bool {
        let entry = self.find_entry(&target);
        entry
            .as_ref()
            .borrow()
            .key
            .map_or(false, |key| key == target)
    }

    pub fn add(&mut self, num: T) {
        let mut before_entry = self.find_entry(&num);
        let mut next_entry = before_entry.as_ref().borrow().right.clone();
        let mut new_entry = Any(Rc::new(RefCell::new(Entry::new(Some(num)))));
        let mut previous = Empty;
        let mut level = 1;

        loop {
            new_entry.as_ref().borrow_mut().left = before_entry.clone();
            new_entry.as_ref().borrow_mut().right = next_entry.clone();
            new_entry.as_ref().borrow_mut().down = previous.clone();
            before_entry.as_ref().borrow_mut().right = new_entry.clone();
            next_entry.as_ref().borrow_mut().left = new_entry.clone();

            if rand::random::<bool>() {
                if level == self.height {
                    let new_head = Rc::new(RefCell::new(Entry::new(None)));
                    let new_tail = Rc::new(RefCell::new(Entry::new(None)));

                    new_head.borrow_mut().right = Any(Rc::clone(&new_tail));
                    new_head.borrow_mut().down = self.head.clone();
                    self.head.as_ref().borrow_mut().up = Any(Rc::clone(&new_head));

                    new_tail.borrow_mut().left = Any(Rc::clone(&new_head));
                    new_tail.borrow_mut().down = self.tail.clone();
                    self.tail.as_ref().borrow_mut().up = Any(Rc::clone(&new_tail));

                    self.head = Any(Rc::clone(&new_head));
                    self.tail = Any(Rc::clone(&new_tail));
                    self.height += 1;
                }

                while before_entry.as_ref().borrow().up.is_none() {
                    before_entry = before_entry.as_ref().borrow().left.clone();
                }
                before_entry = before_entry.as_ref().borrow().up.clone();

                while next_entry.as_ref().borrow().up.is_none() {
                    next_entry = next_entry.as_ref().borrow().right.clone();
                }
                next_entry = next_entry.as_ref().borrow().up.clone();

                level += 1;
                previous = new_entry;
                new_entry = Any(Rc::new(RefCell::new(Entry::new(Some(num)))));
                previous.as_ref().borrow_mut().up = new_entry.clone();
            } else {
                break;
            }
        }
    }

    pub fn erase(&mut self, num: T) -> bool {
        let mut entry = self.find_entry(&num);
        if !entry.as_ref().borrow().key.map_or(false, |key| key == num) {
            return false;
        }

        while entry.is_some() {
            let up = entry.as_ref().borrow().up.clone();
            let left = entry.as_ref().borrow().left.clone();
            let right = entry.as_ref().borrow().right.clone();

            left.as_ref().borrow_mut().right = right.clone();
            right.as_ref().borrow_mut().left = left.clone();

            entry.as_ref().borrow_mut().down = Empty;
            entry.as_ref().borrow_mut().up = Empty;
            entry = up.clone();
        }

        true
    }

    fn find_entry(&self, target: &T) -> EntryRef<T> {
        let mut node = self.head.as_ref();

        loop {
            let node_ref = Rc::clone(&node);
            let node_value = node_ref.borrow();

            if node_value
                .right
                .as_ref()
                .borrow()
                .key
                .map_or(false, |key| &key <= target)
            {
                node = node_value.right.as_ref();
            } else if node_value.down.is_none() {
                break;
            } else {
                node = node_value.down.as_ref();
            }
        }

        Any(node)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_empty_skip_list_entry() {
        let entry: Entry<i32> = Entry::new(None);
        assert!(entry.key.is_none());
        assert!(entry.left.is_none());
        assert!(entry.right.is_none());
        assert!(entry.up.is_none());
        assert!(entry.down.is_none());
    }

    #[test]
    fn create_skip_list_entry() {
        let entry = Entry::new(Some(5));
        assert_eq!(entry.key, Some(5));
        assert!(entry.left.is_none());
        assert!(entry.right.is_none());
        assert!(entry.up.is_none());
        assert!(entry.down.is_none());
    }

    #[test]
    fn create_skip_list() {
        let skiplist: Skiplist<i32> = Skiplist::new();
        let head = skiplist.head;
        let tail = skiplist.tail;

        assert!(head.is_some());
        assert!(tail.is_some());

        let head = head.as_ref();
        let tail = tail.as_ref();

        assert!(head.borrow().left.is_none());
        assert!(head.borrow().up.is_none());
        assert!(head.borrow().down.is_none());

        assert!(tail.borrow().right.is_none());
        assert!(tail.borrow().up.is_none());
        assert!(tail.borrow().down.is_none());

        assert_eq!(head.borrow().right, Any(Rc::clone(&tail)));
        assert_eq!(tail.borrow().left, Any(Rc::clone(&head)));
    }

    #[test]
    fn search_not_existing_element() {
        let skiplist: Skiplist<i32> = Skiplist::new();
        assert_eq!(skiplist.search(5), false);
        assert_eq!(skiplist.search(37), false);
        assert_eq!(skiplist.search(i32::min_value()), false);
        assert_eq!(skiplist.search(i32::max_value()), false);
    }

    #[test]
    fn add_few_elements() {
        let mut skiplist: Skiplist<i32> = Skiplist::new();

        assert_eq!(skiplist.search(5), false);
        skiplist.add(5);
        assert_eq!(skiplist.search(5), true);

        assert_eq!(skiplist.search(i32::min_value()), false);
        skiplist.add(i32::min_value());
        assert_eq!(skiplist.search(i32::min_value()), true);

        assert_eq!(skiplist.search(i32::max_value()), false);
        skiplist.add(i32::max_value());
        assert_eq!(skiplist.search(i32::max_value()), true);
    }

    #[test]
    fn erase_not_existing() {
        let mut skiplist: Skiplist<i32> = Skiplist::new();

        assert_eq!(skiplist.erase(5), false);
        assert_eq!(skiplist.erase(i32::max_value()), false);
        assert_eq!(skiplist.erase(i32::min_value()), false);
    }

    #[test]
    fn erase_existing() {
        let mut skiplist: Skiplist<i32> = Skiplist::new();

        skiplist.add(5);
        skiplist.add(7);
        skiplist.add(i32::max_value());
        skiplist.add(i32::min_value());

        assert_eq!(skiplist.erase(5), true);
        assert_eq!(skiplist.erase(i32::max_value()), true);
        assert_eq!(skiplist.erase(i32::min_value()), true);

        assert_eq!(skiplist.search(5), false);
        assert_eq!(skiplist.search(7), true);
        assert_eq!(skiplist.search(i32::max_value()), false);
        assert_eq!(skiplist.search(i32::min_value()), false);
    }

    #[test]
    fn add_many_elements() {
        let mut skiplist: Skiplist<i32> = Skiplist::new();
        let elements: [i32; 30] = rand::random();

        for &element in elements.iter() {
            skiplist.add(element);
            assert!(skiplist.search(element));
        }

        for &element in elements.iter() {
            assert!(skiplist.search(element));
        }
    }
}