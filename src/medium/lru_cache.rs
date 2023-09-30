#![allow(dead_code)]

use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// A struct representing a node in a doubly linked list used in the LRU cache implementation.
#[derive(Debug)]
struct ListNode {
    key: i32,
    value: i32,
    prev: Option<Rc<RefCell<ListNode>>>,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    /// Creates a new instance of the `LRUCache` struct with the given `key` and `value`.
    pub fn new(key: i32, value: i32) -> Self {
        Self {
            key,
            value,
            prev: None,
            next: None,
        }
    }
}

/// A struct representing a doubly linked list node.
#[derive(Debug)]
struct DoubleListNode {
    head: Option<Rc<RefCell<ListNode>>>,
    tail: Option<Rc<RefCell<ListNode>>>,
}

impl DoubleListNode {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn get_head(&self) -> Option<Rc<RefCell<ListNode>>> {
        if self.head.is_none() {
            None
        } else {
            Some(self.head.as_ref().unwrap().clone())
        }
    }

    fn get_tail(&self) -> Option<Rc<RefCell<ListNode>>> {
        if self.tail.is_none() {
            None
        } else {
            Some(self.tail.as_ref().unwrap().clone())
        }
    }

    pub fn add_front(&mut self, key: i32, value: i32) {
        let node = Rc::new(RefCell::new(ListNode {
            key,
            value,
            prev: None,
            next: self.get_head(),
        }));
        self.head.replace(node);
    }

    pub fn add_back(&mut self, key: i32, value: i32) {
        let node = Rc::new(RefCell::new(ListNode {
            key,
            value,
            prev: self.get_tail(),
            next: None,
        }));
        self.tail.replace(node);
    }

    pub fn add_front_node(&mut self, node: Rc<RefCell<ListNode>>) {
        let head = self.get_head();
        if head.is_some() {
            head.as_ref().unwrap().borrow_mut().prev = Some(node.clone());
        }

        node.borrow_mut().prev = None;
        node.borrow_mut().next = head;

        self.head = Some(node);
    }

    pub fn add_back_node(&mut self, node: Rc<RefCell<ListNode>>) {
        let tail = self.get_tail();
        if tail.is_some() {
            tail.as_ref().unwrap().borrow_mut().next = Some(node.clone());
        }

        node.borrow_mut().prev = tail;
        node.borrow_mut().next = None;

        self.tail = Some(node);
    }

    pub fn remove(&mut self, target: Rc<RefCell<ListNode>>) {
        let prev = target.borrow().prev.clone();
        let next = target.borrow().next.clone();

        match (prev, next) {
            (Some(prev), Some(next)) => {
                prev.borrow_mut().next = Some(next.clone());
                next.borrow_mut().prev = Some(prev);
            }
            (Some(prev), None) => {
                // tail case
                prev.borrow_mut().next.take();
                self.tail.replace(prev);
            }
            (None, Some(next)) => {
                // head case
                next.borrow_mut().prev.take();
                self.head.replace(next);
            }
            (None, None) => {
                // singal node case
                self.head.take();
                self.tail.take();
            }
        }
    }

    pub fn move_head(&mut self, target: Rc<RefCell<ListNode>>) {
        if !Rc::ptr_eq(self.get_head().as_ref().unwrap(), &target) {
            self.remove(target.clone());
            self.add_front_node(target);
        }
    }

    pub fn move_tail(&mut self, target: Rc<RefCell<ListNode>>) {
        if !Rc::ptr_eq(self.get_tail().as_ref().unwrap(), &target) {
            self.remove(target.clone());
            self.add_back_node(target);
        }
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
pub struct LRUCache {
    map: HashMap<i32, Rc<RefCell<ListNode>>>,
    lru: DoubleListNode,
    cap: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            map: HashMap::new(),
            lru: DoubleListNode::new(),
            cap: capacity as usize,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if self.map.contains_key(&key) {
            let node = self.map.get(&key).unwrap();
            self.lru.move_head(node.clone());
            node.as_ref().borrow().value
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let node = if self.map.contains_key(&key) {
            let node = self.map.get(&key).unwrap();
            node.borrow_mut().value = value;
            self.lru.remove(node.clone());
            self.lru.add_front_node(node.clone());
            node.clone()
        } else {
            let node = Rc::new(RefCell::new(ListNode::new(key, value)));
            if self.map.len() == self.cap {
                let tail = self.lru.get_tail().as_ref().unwrap().clone();
                self.map.remove(&tail.as_ref().borrow().key);
                self.lru.remove(tail);

                self.map.insert(key, node.clone());
                self.lru.add_front_node(node.clone());
            } else {
                self.map.insert(key, node.clone());
                self.lru.add_front_node(node.clone());
            }
            node
        };
        if self.lru.tail.is_none() {
            self.lru.add_back_node(node);
        }
    }
}

/*
    Algorithm - Using a doubly linked list and a hashmap

    Time    O(1)
    Space   O(n)
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_146() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3);
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }
}
