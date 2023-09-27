#![allow(dead_code)]
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, Debug)]
pub struct Node {
    pub val: i32,
    pub next: Option<Rc<RefCell<Node>>>,
    pub random: Option<Rc<RefCell<Node>>>,
}

impl Node {
    #[inline]
    pub fn new(val: i32) -> Self {
        Node {
            next: None,
            random: None,
            val,
        }
    }
}

// Solution Semi Correct
pub fn copy_random_list(head: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
    if head.is_none() {
        return None;
    }

    let head = head.unwrap();

    let mut old_new_mapping: HashMap<usize, Rc<RefCell<Node>>> = HashMap::new();
    let mut new_nodes: HashMap<usize, Rc<RefCell<Node>>> = HashMap::new();
    let mut current = Some(Rc::clone(&head));

    while let Some(node) = current {
        let id = node.as_ref() as *const _ as usize;

        let node_value = node.borrow().val;
        let cloned_node = Rc::new(RefCell::new(Node::new(node_value)));
        old_new_mapping.insert(id, cloned_node);

        current = match node.borrow_mut().next {
            Some(ref next_node) => Some(Rc::clone(next_node)),
            None => None,
        };
    }

    current = Some(Rc::clone(&head));
    while let Some(node) = current {
        let id: usize = node.as_ref() as *const _ as usize;
        let cloned_node = old_new_mapping.remove(&id).unwrap();

        if let Some(next_node) = &node.borrow().next {
            let id = next_node.as_ref() as *const _ as usize;
            cloned_node.borrow_mut().next = Some({
                if let Some(new_node) = new_nodes.get(&id) {
                    new_node.clone()
                } else {
                    let new_node = old_new_mapping.get(&id).unwrap();
                    new_nodes.insert(id, new_node.clone());
                    new_node.clone()
                }
            });
        }

        if let Some(random_node) = &node.borrow().random {
            let id = random_node.as_ref() as *const _ as usize;
            cloned_node.borrow_mut().random = Some({
                if let Some(new_node) = new_nodes.get(&id) {
                    new_node.clone()
                } else {
                    let new_node = old_new_mapping.get(&id).unwrap().clone();
                    new_nodes.insert(id, new_node.clone());
                    new_node
                }
            });
        }

        old_new_mapping.insert(id, cloned_node);

        current = match node.borrow().next.as_ref() {
            Some(ref next_node) => Some(Rc::clone(next_node)),
            None => None,
        };
    }

    old_new_mapping.remove(&(head.as_ref() as *const _ as usize))
}

// Note u can make pull request resolving this issue completely
